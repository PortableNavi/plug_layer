use plug_layer::*;


struct SimpleLayer;
impl LayerDispatch<()> for SimpleLayer {}


impl SimpleLayer
{
    fn echo(&self, num: usize) -> usize
    {
        num
    }
}


#[test]
fn simple_layer()
{
    let mut reg = LayerReg::new();
    reg.insert(SimpleLayer);

    assert_eq!(reg_read!(reg, SimpleLayer).echo(39), 39)
}


#[derive(Clone, Debug, PartialEq, Eq)]
enum LayerEvent
{
    EchoIn(usize),
    Response(EventResponse),
}


#[derive(Clone, Debug, PartialEq, Eq)]
enum EventResponse
{
    EchoOut(usize),
    EchoReceived(usize),
}


struct DispatchLayer;
impl LayerDispatch<LayerEvent> for DispatchLayer
{
    fn dispatch(&mut self, event: &LayerEvent, queue: &mut EventQueue<LayerEvent>)
    {
        match event
        {
            LayerEvent::EchoIn(num) =>
            {
                queue.push(LayerEvent::Response(EventResponse::EchoOut(*num)));
            }

            LayerEvent::Response(EventResponse::EchoOut(num)) =>
            {
                queue.push(LayerEvent::Response(EventResponse::EchoReceived(*num)));
            }

            _ => (),
        }
    }
}


#[test]
fn simple_dispatch_layer()
{
    let mut reg = LayerReg::new();
    reg.insert(DispatchLayer);

    let echo = reg
        .dispatch(LayerEvent::EchoIn(39))
        .slice()
        .contains(&LayerEvent::Response(EventResponse::EchoOut(39)));

    assert!(echo)
}


#[test]
fn redispatch_dispatch_layer()
{
    let mut reg = LayerReg::new();
    reg.insert(DispatchLayer);

    let echo = reg
        .dispatch(LayerEvent::EchoIn(39))
        .dispatch(&mut reg)
        .slice()
        .contains(&LayerEvent::Response(EventResponse::EchoReceived(39)));

    assert!(echo);
}
