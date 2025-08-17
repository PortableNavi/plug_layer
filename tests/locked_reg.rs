use std::sync::LazyLock;

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


#[cfg(not(feature = "tokio"))]
#[test]
fn static_layer()
{
    static REG: LazyLock<LockedReg<()>> = LazyLock::new(LockedReg::new);

    REG.insert(SimpleLayer);
    assert_eq!(locked_read!(REG, SimpleLayer).echo(39), 39)
}


#[cfg(feature = "tokio")]
#[tokio::test]
async fn static_layer()
{
    static REG: LazyLock<LockedReg<()>> = LazyLock::new(LockedReg::new);

    REG.insert(SimpleLayer).await;
    assert_eq!(locked_read!(REG, SimpleLayer).echo(39), 39)
}
