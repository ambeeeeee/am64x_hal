#[doc = "Register `CFG_RX_DLYLINE_CTRL` reader"]
pub type R = crate::R<CfgRxDlylineCtrlSpec>;
#[doc = "Register `CFG_RX_DLYLINE_CTRL` writer"]
pub type W = crate::W<CfgRxDlylineCtrlSpec>;
#[doc = "Field `RXCLK_DLY` reader - 4:0\\]
Delay Line Tap Select for RXCLKThis bitfield selects the number of delay elements inserted into the RXCLK path from the pin boundary to the receiver core. 0h \\[R/W\\]
Zero delay elements are included in the RXCLK path. RXCLK is taken directly from the pin.1h \\[R/W\\]
One delay element is included in the RXCLK path.2h \\[R/W\\]
Two delay elements are included in the RXCLK path....1Fh \\[R/W\\]
31 delay elements are included in the RXCLK path, the maximum."]
pub type RxclkDlyR = crate::FieldReader;
#[doc = "Field `RXCLK_DLY` writer - 4:0\\]
Delay Line Tap Select for RXCLKThis bitfield selects the number of delay elements inserted into the RXCLK path from the pin boundary to the receiver core. 0h \\[R/W\\]
Zero delay elements are included in the RXCLK path. RXCLK is taken directly from the pin.1h \\[R/W\\]
One delay element is included in the RXCLK path.2h \\[R/W\\]
Two delay elements are included in the RXCLK path....1Fh \\[R/W\\]
31 delay elements are included in the RXCLK path, the maximum."]
pub type RxclkDlyW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RXD0_DLY` reader - 9:5\\]
Delay Line Tap Select for RXD0This bitfield selects the number of delay elements inserted into the RXD0 path from the pin boundary to the receiver core. 0h \\[R/W\\]
Zero delay elements are included in the RXD0 path. RXD0 is taken directly from the pin.1h \\[R/W\\]
One delay element is included in the RXD0 path.2h \\[R/W\\]
Two delay elements are included in the RXD0 path....1Fh \\[R/W\\]
31 delay elements are included in the RXD0 path, the maximum."]
pub type Rxd0DlyR = crate::FieldReader;
#[doc = "Field `RXD0_DLY` writer - 9:5\\]
Delay Line Tap Select for RXD0This bitfield selects the number of delay elements inserted into the RXD0 path from the pin boundary to the receiver core. 0h \\[R/W\\]
Zero delay elements are included in the RXD0 path. RXD0 is taken directly from the pin.1h \\[R/W\\]
One delay element is included in the RXD0 path.2h \\[R/W\\]
Two delay elements are included in the RXD0 path....1Fh \\[R/W\\]
31 delay elements are included in the RXD0 path, the maximum."]
pub type Rxd0DlyW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RXD1_DLY` reader - 14:10\\]
Delay Line Tap Select for RXD1This bitfield selects the number of delay elements inserted into the RXD1 path from the pin boundary to the receiver core. 0h \\[R/W\\]
Zero delay elements are included in the RXD1 path. RXD1 is taken directly from the pin.1h \\[R/W\\]
One delay element is included in the RXD1 path.2h \\[R/W\\]
Two delay elements are included in the RXD1 path....1Fh \\[R/W\\]
31 delay elements are included in the RXD1 path, the maximum."]
pub type Rxd1DlyR = crate::FieldReader;
#[doc = "Field `RXD1_DLY` writer - 14:10\\]
Delay Line Tap Select for RXD1This bitfield selects the number of delay elements inserted into the RXD1 path from the pin boundary to the receiver core. 0h \\[R/W\\]
Zero delay elements are included in the RXD1 path. RXD1 is taken directly from the pin.1h \\[R/W\\]
One delay element is included in the RXD1 path.2h \\[R/W\\]
Two delay elements are included in the RXD1 path....1Fh \\[R/W\\]
31 delay elements are included in the RXD1 path, the maximum."]
pub type Rxd1DlyW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Delay Line Tap Select for RXCLKThis bitfield selects the number of delay elements inserted into the RXCLK path from the pin boundary to the receiver core. 0h \\[R/W\\]
Zero delay elements are included in the RXCLK path. RXCLK is taken directly from the pin.1h \\[R/W\\]
One delay element is included in the RXCLK path.2h \\[R/W\\]
Two delay elements are included in the RXCLK path....1Fh \\[R/W\\]
31 delay elements are included in the RXCLK path, the maximum."]
    #[inline(always)]
    pub fn rxclk_dly(&self) -> RxclkDlyR {
        RxclkDlyR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - 9:5\\]
Delay Line Tap Select for RXD0This bitfield selects the number of delay elements inserted into the RXD0 path from the pin boundary to the receiver core. 0h \\[R/W\\]
Zero delay elements are included in the RXD0 path. RXD0 is taken directly from the pin.1h \\[R/W\\]
One delay element is included in the RXD0 path.2h \\[R/W\\]
Two delay elements are included in the RXD0 path....1Fh \\[R/W\\]
31 delay elements are included in the RXD0 path, the maximum."]
    #[inline(always)]
    pub fn rxd0_dly(&self) -> Rxd0DlyR {
        Rxd0DlyR::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - 14:10\\]
Delay Line Tap Select for RXD1This bitfield selects the number of delay elements inserted into the RXD1 path from the pin boundary to the receiver core. 0h \\[R/W\\]
Zero delay elements are included in the RXD1 path. RXD1 is taken directly from the pin.1h \\[R/W\\]
One delay element is included in the RXD1 path.2h \\[R/W\\]
Two delay elements are included in the RXD1 path....1Fh \\[R/W\\]
31 delay elements are included in the RXD1 path, the maximum."]
    #[inline(always)]
    pub fn rxd1_dly(&self) -> Rxd1DlyR {
        Rxd1DlyR::new(((self.bits >> 10) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Delay Line Tap Select for RXCLKThis bitfield selects the number of delay elements inserted into the RXCLK path from the pin boundary to the receiver core. 0h \\[R/W\\]
Zero delay elements are included in the RXCLK path. RXCLK is taken directly from the pin.1h \\[R/W\\]
One delay element is included in the RXCLK path.2h \\[R/W\\]
Two delay elements are included in the RXCLK path....1Fh \\[R/W\\]
31 delay elements are included in the RXCLK path, the maximum."]
    #[inline(always)]
    #[must_use]
    pub fn rxclk_dly(&mut self) -> RxclkDlyW<CfgRxDlylineCtrlSpec> {
        RxclkDlyW::new(self, 0)
    }
    #[doc = "Bits 5:9 - 9:5\\]
Delay Line Tap Select for RXD0This bitfield selects the number of delay elements inserted into the RXD0 path from the pin boundary to the receiver core. 0h \\[R/W\\]
Zero delay elements are included in the RXD0 path. RXD0 is taken directly from the pin.1h \\[R/W\\]
One delay element is included in the RXD0 path.2h \\[R/W\\]
Two delay elements are included in the RXD0 path....1Fh \\[R/W\\]
31 delay elements are included in the RXD0 path, the maximum."]
    #[inline(always)]
    #[must_use]
    pub fn rxd0_dly(&mut self) -> Rxd0DlyW<CfgRxDlylineCtrlSpec> {
        Rxd0DlyW::new(self, 5)
    }
    #[doc = "Bits 10:14 - 14:10\\]
Delay Line Tap Select for RXD1This bitfield selects the number of delay elements inserted into the RXD1 path from the pin boundary to the receiver core. 0h \\[R/W\\]
Zero delay elements are included in the RXD1 path. RXD1 is taken directly from the pin.1h \\[R/W\\]
One delay element is included in the RXD1 path.2h \\[R/W\\]
Two delay elements are included in the RXD1 path....1Fh \\[R/W\\]
31 delay elements are included in the RXD1 path, the maximum."]
    #[inline(always)]
    #[must_use]
    pub fn rxd1_dly(&mut self) -> Rxd1DlyW<CfgRxDlylineCtrlSpec> {
        Rxd1DlyW::new(self, 10)
    }
}
#[doc = "Receive delay line control register. Protected by LOCK field in RX_LOCK_CTRL register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_rx_dlyline_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_rx_dlyline_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgRxDlylineCtrlSpec;
impl crate::RegisterSpec for CfgRxDlylineCtrlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cfg_rx_dlyline_ctrl::R`](R) reader structure"]
impl crate::Readable for CfgRxDlylineCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_rx_dlyline_ctrl::W`](W) writer structure"]
impl crate::Writable for CfgRxDlylineCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CFG_RX_DLYLINE_CTRL to value 0"]
impl crate::Resettable for CfgRxDlylineCtrlSpec {
    const RESET_VALUE: u16 = 0;
}
