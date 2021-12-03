//! Traits to implement non-blocking tasks for the async runtime.
//!
//! In addition to blocking tasks, the async runtime supports non-blocking tasks which fall into
//! two categories: tasks that can be called once implement [`AsyncTask`], tasks that can be
//! called multiple times implement [`PersistentTask`].
//!
//! Both of these traits require that you implement one or more async methods. Rather than a
//! mutable reference to a [`GcFrame`] they take a mutable reference to an [`AsyncGcFrame`].
//! This frame type provides the same functionality as `GcFrame`, and can be used in combination
//! with several async methods. Most importantly, the methods of the trait [`CallAsync`] which let
//! you schedule a Julia function call as a new Julia task and await its completion.
//!
//! [`GcFrame`]: crate::memory::frame::GcFrame
//! [`CallAsync`]: crate::extensions::multitask::call_async::CallAsync

use std::fmt;
use std::marker::PhantomData;
use std::sync::Arc;

use super::async_frame::AsyncGcFrame;
use super::mode::Async;
use super::result_sender::ResultSender;
use super::AsyncStackPage;
use super::RequireSendSync;
use crate::error::{JlrsError, JlrsResult};
use crate::memory::frame::GcFrame;
use crate::memory::global::Global;
use crate::memory::stack_page::StackPage;
use async_trait::async_trait;

#[cfg(feature = "async-std-rt")]
use crate::extensions::multitask::runtime::async_std_rt::{channel, HandleSender};
#[cfg(feature = "tokio-rt")]
use crate::extensions::multitask::runtime::tokio_rt::{channel, HandleSender};

/// A task that returns once. In order to schedule the task you must use [`AsyncJulia::task`] or
/// [`AsyncJulia::try_task`].
///
/// [`AsyncJulia::task`]: crate::extensions::multitask::AsyncJulia::task
/// [`AsyncJulia::try_task`]: crate::extensions::multitask::AsyncJulia::try_task
#[async_trait(?Send)]
pub trait AsyncTask: 'static + Send + Sync {
    /// The type of the result which is returned if `run` completes successfully.
    type Output: 'static + Send + Sync;

    /// The number of slots preallocated for the `AsyncGcFrame` provided to `run`.
    const RUN_SLOTS: usize = 0;

    /// The number of slots preallocated for the `AsyncGcFrame` provided to `register`.
    const REGISTER_SLOTS: usize = 0;

    /// Register the task. Note that this method is not called automatically, but only if
    /// [`AsyncJulia::register_task`] or [`AsyncJulia::try_register_task`] is used. This method
    /// can be implemented to take care of everything required to execute the task successfully,
    /// like loading packages.
    ///
    /// [`AsyncJulia::register_task`]: crate::extensions::multitask::AsyncJulia::register_task
    /// [`AsyncJulia::try_register_task`]: crate::extensions::multitask::AsyncJulia::try_register_task
    async fn register<'frame>(
        _global: Global<'frame>,
        _frame: &mut AsyncGcFrame<'frame>,
    ) -> JlrsResult<()> {
        Ok(())
    }

    /// Run this task. This method takes a `Global` and a mutable reference to an `AsyncGcFrame`,
    /// which lets you interact with Julia.
    async fn run<'frame>(
        &mut self,
        global: Global<'frame>,
        frame: &mut AsyncGcFrame<'frame>,
    ) -> JlrsResult<Self::Output>;
}

/// A task that can be called multiple times. In order to schedule the task you must use
/// [`AsyncJulia::persistent`] or [`AsyncJulia::try_persistent`].
///
/// [`AsyncJulia::persistent`]: crate::extensions::multitask::AsyncJulia::persistent
/// [`AsyncJulia::try_persistent`]: crate::extensions::multitask::AsyncJulia::try_persistent
#[async_trait(?Send)]
pub trait PersistentTask: 'static + Send + Sync {
    /// The type of the result which is returned if `init` completes successfully. This data is
    /// provided to every call of `run`. Because `init` takes a frame with the `'static` lifetime,
    /// this type can contain Julia data.
    type State: 'static;

    /// The type of the data that must be provided when calling this persistent through its handle.
    type Input: 'static + Send + Sync;

    /// The type of the result which is returned if `run` completes successfully.
    type Output: 'static + Send + Sync;

    /// The capacity of the channel the [`PersistentHandle`] uses to communicate with this
    /// persistent.
    ///
    /// If it's set to 0, the channel is unbounded.
    const CHANNEL_CAPACITY: usize = 0;

    /// The number of slots preallocated for the `AsyncGcFrame` provided to `register`.
    const REGISTER_SLOTS: usize = 0;

    /// The number of slots preallocated for the `AsyncGcFrame` provided to `init`.
    const INIT_SLOTS: usize = 0;

    /// The number of slots preallocated for the `AsyncGcFrame` provided to `run`.
    const RUN_SLOTS: usize = 0;

    // NB: `init` and `run` have an explicit 'inner lifetime . If this lifetime is elided
    // `PersistentTask`s can be implemented in bin crates but not in lib crates (rustc 1.54.0)

    /// Register this persistent. Note that this method is not called automatically, but only if
    /// [`AsyncJulia::register_persistent`] or [`AsyncJulia::try_register_persistent`] is used. This
    /// method can be implemented to take care of everything required to execute the task
    /// successfully, like loading packages.
    ///
    /// [`AsyncJulia::register_persistent`]: crate::extensions::multitask::AsyncJulia::register_persistent
    /// [`AsyncJulia::try_register_persistent`]: crate::extensions::multitask::AsyncJulia::try_register_persistent
    async fn register<'frame>(
        _global: Global<'frame>,
        _frame: &mut AsyncGcFrame<'frame>,
    ) -> JlrsResult<()> {
        Ok(())
    }

    /// Initialize the task. You can interact with Julia inside this method, the frame is
    /// not dropped until the task itself is dropped. This means that `State` can contain
    /// arbitrary Julia data rooted in this frame. This data is provided to every call to `run`.
    async fn init<'inner>(
        &'inner mut self,
        global: Global<'static>,
        frame: &'inner mut AsyncGcFrame<'static>,
    ) -> JlrsResult<Self::State>;

    /// Run the task. This method takes a `Global` and a mutable reference to an
    /// `AsyncGcFrame`, which lets you interact with Julia. It's also provided with a mutable
    /// reference to its `state` and the `input` provided by the caller. While the state is
    /// mutable, it's not possible to allocate a new Julia value in `run` and assign it to the
    /// state because the frame doesn't live long enough.
    async fn run<'inner, 'frame>(
        &'inner mut self,
        global: Global<'frame>,
        frame: &'inner mut AsyncGcFrame<'frame>,
        state: &'inner mut Self::State,
        input: Self::Input,
    ) -> JlrsResult<Self::Output>;

    async fn exit<'inner>(
        &'inner mut self,
        _global: Global<'static>,
        _frame: &'inner mut AsyncGcFrame<'static>,
        _state: &'inner mut Self::State,
    ) {
    }
}

pub(crate) struct PersistentMessage<GT>
where
    GT: PersistentTask,
{
    msg: InnerPersistentMessage<GT>,
}

impl<GT> fmt::Debug for PersistentMessage<GT>
where
    GT: PersistentTask,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("PersistentMessage")
    }
}

type InnerPersistentMessage<GT> = Box<
    dyn GenericCallPersistentMessage<
        Input = <GT as PersistentTask>::Input,
        Output = <GT as PersistentTask>::Output,
    >,
>;

/// A handle to a [`PersistentTask`]. This handle can be used to call the task and shared
/// across threads. The `PersistentTask` is dropped when its final handle has been dropped and all
/// remaining pending calls have completed.
#[derive(Clone)]
pub struct PersistentHandle<GT>
where
    GT: PersistentTask,
{
    sender: HandleSender<GT>,
}

impl<GT> PersistentHandle<GT>
where
    GT: PersistentTask,
{
    fn new(sender: HandleSender<GT>) -> Self {
        PersistentHandle { sender }
    }

    /// Call the task, this method waits until there's room available in the channel.
    pub async fn call<R>(&self, input: GT::Input, sender: R)
    where
        R: ResultSender<JlrsResult<GT::Output>>,
    {
        self.sender
            .send(PersistentMessage {
                msg: Box::new(CallPersistentMessage {
                    input: Some(input),
                    sender,
                    _marker: PhantomData,
                }),
            })
            .await
            .expect("Channel was closed")
    }

    /// Call the task, this method returns an error immediately if there's NO room available
    /// in the channel.
    pub fn try_call<R>(&self, input: GT::Input, sender: R) -> JlrsResult<()>
    where
        R: ResultSender<JlrsResult<GT::Output>>,
    {
        match self.sender.try_send(PersistentMessage {
            msg: Box::new(CallPersistentMessage {
                input: Some(input),
                sender,
                _marker: PhantomData,
            }),
        }) {
            Ok(_) => Ok(()),
            Err(e) => Err(JlrsError::other(e))?,
        }
    }
}

// Ensure the handle can be shared across threads
impl<GT: PersistentTask> RequireSendSync for PersistentHandle<GT> {}

// What follows is a significant amount of indirection to allow different tasks to have a
// different Output, and allow users to provide an arbitrary sender that implements ReturnChannel
// to return some result.
pub(crate) enum Task {}
pub(crate) enum RegisterTask {}
pub(crate) enum Persistent {}
pub(crate) enum RegisterPersistent {}

struct CallPersistentMessage<I, O, RC>
where
    I: Send + Sync,
    O: Send + Sync + 'static,
    RC: ResultSender<JlrsResult<O>>,
{
    sender: RC,
    input: Option<I>,
    _marker: PhantomData<O>,
}

#[async_trait(?Send)]
trait GenericCallPersistentMessage: Send + Sync {
    type Input;
    type Output;

    async fn respond(self: Box<Self>, result: JlrsResult<Self::Output>);
    fn input(&mut self) -> Self::Input;
}

#[async_trait(?Send)]
impl<I, O, RC> GenericCallPersistentMessage for CallPersistentMessage<I, O, RC>
where
    I: Send + Sync,
    O: Send + Sync,
    RC: ResultSender<JlrsResult<O>>,
{
    type Input = I;
    type Output = O;

    async fn respond(self: Box<Self>, result: JlrsResult<Self::Output>) {
        Box::new(self.sender).send(result).await
    }

    fn input(&mut self) -> Self::Input {
        self.input.take().unwrap()
    }
}

#[async_trait(?Send)]
trait GenericAsyncTask: Send + Sync {
    type AT: AsyncTask + Send + Sync;

    async fn call_run<'inner>(
        &'inner mut self,
        global: Global<'static>,
        frame: &'inner mut AsyncGcFrame<'static>,
    ) -> JlrsResult<<Self::AT as AsyncTask>::Output>;
}

#[async_trait(?Send)]
impl<AT: AsyncTask> GenericAsyncTask for AT {
    type AT = Self;
    async fn call_run<'inner>(
        &'inner mut self,
        global: Global<'static>,
        frame: &'inner mut AsyncGcFrame<'static>,
    ) -> JlrsResult<<Self::AT as AsyncTask>::Output> {
        self.run(global, frame).await
    }
}

trait GenericRegisterAsyncTask: Send + Sync {
    type AT: AsyncTask + Send + Sync;
}

impl<AT: AsyncTask> GenericRegisterAsyncTask for AT {
    type AT = Self;
}

#[async_trait(?Send)]
trait GenericPersistentTask: Send + Sync {
    type GT: PersistentTask + Send + Sync;

    async unsafe fn call_init<'inner>(
        &'inner mut self,
        global: Global<'static>,
        frame: &'inner mut AsyncGcFrame<'static>,
    ) -> JlrsResult<<Self::GT as PersistentTask>::State>;

    async unsafe fn call_run<'inner>(
        &'inner mut self,
        global: Global<'static>,
        frame: &'inner mut AsyncGcFrame<'static>,
        state: &'inner mut <Self::GT as PersistentTask>::State,
        input: <Self::GT as PersistentTask>::Input,
    ) -> JlrsResult<<Self::GT as PersistentTask>::Output>;

    fn create_handle(&self, sender: HandleSender<Self::GT>) -> PersistentHandle<Self::GT>;
}

#[async_trait(?Send)]
impl<GT> GenericPersistentTask for GT
where
    GT: PersistentTask,
{
    type GT = Self;

    async unsafe fn call_init<'inner>(
        &'inner mut self,
        global: Global<'static>,
        frame: &'inner mut AsyncGcFrame<'static>,
    ) -> JlrsResult<<Self::GT as PersistentTask>::State> {
        {
            self.init(global, frame).await
        }
    }

    async unsafe fn call_run<'inner>(
        &'inner mut self,
        global: Global<'static>,
        frame: &'inner mut AsyncGcFrame<'static>,
        state: &'inner mut <Self::GT as PersistentTask>::State,
        input: <Self::GT as PersistentTask>::Input,
    ) -> JlrsResult<<Self::GT as PersistentTask>::Output> {
        unsafe {
            let output = {
                let mut nested = frame.nest_async(Self::RUN_SLOTS);
                self.run(global, &mut nested, state, input).await
            };

            output
        }
    }

    fn create_handle(&self, sender: HandleSender<Self>) -> PersistentHandle<Self> {
        PersistentHandle::new(sender)
    }
}

trait GenericRegisterPersistentTask: Send + Sync {
    type GT: PersistentTask + Send + Sync;
}

impl<GT: PersistentTask> GenericRegisterPersistentTask for GT {
    type GT = Self;
}

pub(crate) struct PendingTask<RC, T, Kind> {
    task: Option<T>,
    sender: RC,
    _kind: PhantomData<Kind>,
}

impl<RC, AT> PendingTask<RC, AT, Task>
where
    RC: ResultSender<JlrsResult<AT::Output>>,
    AT: AsyncTask,
{
    pub(super) fn new(task: AT, sender: RC) -> Self {
        PendingTask {
            task: Some(task),
            sender,
            _kind: PhantomData,
        }
    }

    fn split(self) -> (AT, RC) {
        (self.task.unwrap(), self.sender)
    }
}

impl<IRC, GT> PendingTask<IRC, GT, Persistent>
where
    IRC: ResultSender<JlrsResult<PersistentHandle<GT>>>,
    GT: PersistentTask,
{
    pub(super) fn new(task: GT, sender: IRC) -> Self {
        PendingTask {
            task: Some(task),
            sender,
            _kind: PhantomData,
        }
    }

    fn split(self) -> (GT, IRC) {
        (self.task.unwrap(), self.sender)
    }
}

impl<RC, AT> PendingTask<RC, AT, RegisterTask>
where
    RC: ResultSender<JlrsResult<()>>,
    AT: AsyncTask,
{
    pub(super) fn new(sender: RC) -> Self {
        PendingTask {
            task: None,
            sender,
            _kind: PhantomData,
        }
    }

    fn sender(self) -> RC {
        self.sender
    }
}

impl<RC, GT> PendingTask<RC, GT, RegisterPersistent>
where
    RC: ResultSender<JlrsResult<()>>,
    GT: PersistentTask,
{
    pub(super) fn new(sender: RC) -> Self {
        PendingTask {
            task: None,
            sender,
            _kind: PhantomData,
        }
    }

    fn sender(self) -> RC {
        self.sender
    }
}

#[async_trait(?Send)]
pub(crate) trait GenericPendingTask: Send + Sync {
    async fn call(mut self: Box<Self>, mut stack: &mut AsyncStackPage);
}

#[async_trait(?Send)]
impl<RC, AT> GenericPendingTask for PendingTask<RC, AT, Task>
where
    RC: ResultSender<JlrsResult<AT::Output>>,
    AT: AsyncTask,
{
    async fn call(mut self: Box<Self>, mut stack: &mut AsyncStackPage) {
        unsafe {
            let (mut task, result_sender) = self.split();

            // Transmute to get static lifetimes. Should be okay because tasks can't leak
            // Julia data and the frame is not dropped until the task is dropped.
            let mode = Async(std::mem::transmute(&stack.top[1]));
            if stack.page.size() < AT::RUN_SLOTS + 2 {
                stack.page = StackPage::new(AT::RUN_SLOTS + 2);
            }
            let raw = std::mem::transmute(stack.page.as_mut());
            let mut frame = AsyncGcFrame::new(raw, mode);
            let global = Global::new();

            let res = task.call_run(global, &mut frame).await;
            Box::new(result_sender).send(res).await;
        }
    }
}

#[async_trait(?Send)]
impl<RC, AT> GenericPendingTask for PendingTask<RC, AT, RegisterTask>
where
    RC: ResultSender<JlrsResult<()>>,
    AT: AsyncTask,
{
    async fn call(mut self: Box<Self>, mut stack: &mut AsyncStackPage) {
        unsafe {
            let sender = self.sender();

            let mode = Async(&stack.top[1]);
            if stack.page.size() < AT::REGISTER_SLOTS + 2 {
                stack.page = StackPage::new(AT::REGISTER_SLOTS + 2);
            }

            let raw = stack.page.as_mut();
            let mut frame = AsyncGcFrame::new(raw, mode);
            let global = Global::new();

            let res = AT::register(global, &mut frame).await;
            Box::new(sender).send(res).await;
        }
    }
}

#[async_trait(?Send)]
impl<RC, GT> GenericPendingTask for PendingTask<RC, GT, RegisterPersistent>
where
    RC: ResultSender<JlrsResult<()>>,
    GT: PersistentTask,
{
    async fn call(mut self: Box<Self>, mut stack: &mut AsyncStackPage) {
        unsafe {
            let sender = self.sender();

            let mode = Async(&stack.top[1]);
            if stack.page.size() < GT::REGISTER_SLOTS + 2 {
                stack.page = StackPage::new(GT::REGISTER_SLOTS + 2);
            }

            let raw = stack.page.as_mut();
            let mut frame = AsyncGcFrame::new(raw, mode);
            let global = Global::new();

            let res = GT::register(global, &mut frame).await;
            Box::new(sender).send(res).await;
        }
    }
}

#[async_trait(?Send)]
impl<IRC, GT> GenericPendingTask for PendingTask<IRC, GT, Persistent>
where
    IRC: ResultSender<JlrsResult<PersistentHandle<GT>>>,
    GT: PersistentTask,
{
    async fn call(mut self: Box<Self>, mut stack: &mut AsyncStackPage) {
        unsafe {
            {
                let (mut persistent, handle_sender) = self.split();
                let handle_sender = Box::new(handle_sender);

                // Transmute to get static lifetimes. Should be okay because tasks can't leak
                // Julia data and the frame is not dropped until the task is dropped.
                let mode = Async(std::mem::transmute(&stack.top[1]));
                if stack.page.size() < GT::INIT_SLOTS + 2 {
                    stack.page = StackPage::new(GT::INIT_SLOTS + 2);
                }

                let raw = std::mem::transmute(stack.page.as_mut());
                let mut frame = AsyncGcFrame::new(raw, mode);
                let global = Global::new();

                match persistent.call_init(global, &mut frame).await {
                    Ok(mut state) => {
                        #[allow(unused_mut)]
                        let (sender, mut receiver) = channel(GT::CHANNEL_CAPACITY);

                        let handle = persistent.create_handle(Arc::new(sender));
                        handle_sender.send(Ok(handle)).await;

                        loop {
                            #[cfg(feature = "async-std-rt")]
                            let mut msg = match receiver.recv().await {
                                Ok(msg) => msg.msg,
                                Err(_) => break,
                            };

                            #[cfg(feature = "tokio-rt")]
                            let mut msg = match receiver.recv().await {
                                Some(msg) => msg.msg,
                                None => break,
                            };

                            let res = persistent
                                .call_run(global, &mut frame, &mut state, msg.input())
                                .await;

                            msg.respond(res).await;
                        }

                        persistent.exit(global, &mut frame, &mut state).await;
                    }
                    Err(e) => {
                        handle_sender.send(Err(e)).await;
                    }
                }
            }
        }
    }
}

pub(crate) struct BlockingTask<F, RC, T> {
    func: F,
    sender: RC,
    slots: usize,
    _res: PhantomData<T>,
}

impl<F, RC, T> BlockingTask<F, RC, T>
where
    for<'base> F:
        Send + Sync + FnOnce(Global<'base>, &mut GcFrame<'base, Async<'base>>) -> JlrsResult<T>,
    RC: ResultSender<JlrsResult<T>>,
    T: Send + Sync + 'static,
{
    pub(crate) fn new(func: F, sender: RC, slots: usize) -> Self {
        Self {
            func,
            sender,
            slots,
            _res: PhantomData,
        }
    }

    fn call<'scope>(
        self: Box<Self>,
        frame: &mut GcFrame<'scope, Async<'scope>>,
    ) -> (JlrsResult<T>, RC) {
        let global = unsafe { Global::new() };
        let func = self.func;
        let res = func(global, frame);
        (res, self.sender)
    }
}

pub(crate) trait GenericBlockingTask: Send + Sync {
    fn call(self: Box<Self>, stack: &mut AsyncStackPage);
}

impl<F, RC, T> GenericBlockingTask for BlockingTask<F, RC, T>
where
    for<'base> F:
        Send + Sync + FnOnce(Global<'base>, &mut GcFrame<'base, Async<'base>>) -> JlrsResult<T>,
    RC: ResultSender<JlrsResult<T>>,
    T: Send + Sync + 'static,
{
    fn call(self: Box<Self>, stack: &mut AsyncStackPage) {
        let mode = Async(&stack.top[1]);
        if stack.page.size() < self.slots + 2 {
            stack.page = StackPage::new(self.slots + 2);
        }
        let raw = stack.page.as_mut();
        let mut frame = unsafe { GcFrame::new(raw, mode) };
        let (res, ch) = self.call(&mut frame);

        #[cfg(feature = "tokio-rt")]
        {
            tokio::task::spawn_local(async {
                Box::new(ch).send(res).await;
            });
        }

        #[cfg(feature = "async-std-rt")]
        {
            async_std::task::spawn_local(async {
                Box::new(ch).send(res).await;
            });
        }
    }
}
