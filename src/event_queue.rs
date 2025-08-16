use crate::LayerReg;


#[derive(Clone)]
pub struct EventQueue<E>(Vec<E>);
impl<E> EventQueue<E>
where
    E: std::fmt::Debug + Clone,
{
    pub fn push(&mut self, event: impl Into<E>)
    {
        self.0.push(event.into());
    }

    pub fn slice(&self) -> &[E]
    {
        &self.0
    }

    pub fn dispatch(self, on: &mut LayerReg<E>) -> Self
    {
        debug!(
            "Dispatching Reactions: {:?} to LayerEvent dispatch...",
            &self.0
        );

        const MAX_ITERATIONS: usize = 5;

        let mut current = self;
        let mut iterations = 0;
        let mut answers = Self::default();

        while !current.0.is_empty()
        {
            if iterations >= MAX_ITERATIONS
            {
                error!(
                    "Dispatch of layer event reactions seems to be looping, stopping dispatch now."
                );

                break;
            }

            let mut queue = Self::default();

            for event in current.0
            {
                queue.0.extend(on.dispatch(event).0);
            }

            current = queue;
            answers.0.extend(current.0.clone());
            iterations += 1;
        }

        debug!("Dispatch finished");

        answers
    }
}


impl<E> Default for EventQueue<E>
{
    fn default() -> Self
    {
        Self(Vec::new())
    }
}
