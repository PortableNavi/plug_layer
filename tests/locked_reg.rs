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


#[test]
fn static_layer()
{
    static REG: LazyLock<LockedReg<()>> = LazyLock::new(LockedReg::new);

    REG.insert(SimpleLayer);
    assert_eq!(reg_read!(REG, SimpleLayer).echo(39), 39)
}
