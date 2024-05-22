#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_329` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl329Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_329` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl329Spec>;
#[doc = "Field `DEVICE1_BYTE0_CS1` reader - 1:0\\]
Defines the byte location of byte0 in the memory datapath for device 1 on chip 1. Used for MRRs to identify where data will be returned. DEV=1"]
pub type Device1Byte0Cs1R = crate::FieldReader;
#[doc = "Field `DEVICE1_BYTE0_CS1` writer - 1:0\\]
Defines the byte location of byte0 in the memory datapath for device 1 on chip 1. Used for MRRs to identify where data will be returned. DEV=1"]
pub type Device1Byte0Cs1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Q_FULLNESS` reader - 12:8\\]
Quantity that determines when the command queue almost full signal will assert \\[q_almost_full\\]. When cleared to 0, the q_almost_full signal will be driven to 0 irrespective of number of entries in the command queue."]
pub type QFullnessR = crate::FieldReader;
#[doc = "Field `Q_FULLNESS` writer - 12:8\\]
Quantity that determines when the command queue almost full signal will assert \\[q_almost_full\\]. When cleared to 0, the q_almost_full signal will be driven to 0 irrespective of number of entries in the command queue."]
pub type QFullnessW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `IN_ORDER_ACCEPT` reader - 16:16\\]
Forces the controller to accept commands in the order in which they are placed in the command queue."]
pub type InOrderAcceptR = crate::BitReader;
#[doc = "Field `IN_ORDER_ACCEPT` writer - 16:16\\]
Forces the controller to accept commands in the order in which they are placed in the command queue."]
pub type InOrderAcceptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WR_ORDER_REQ` reader - 25:24\\]
Determines if the controller can re-order write commands from the same source ID and/or the same port. Bit \\[0\\]
controls source ID usage and bit \\[1\\]
controls port ID usage. Set each bit to 1 to enable usage in placement logic."]
pub type WrOrderReqR = crate::FieldReader;
#[doc = "Field `WR_ORDER_REQ` writer - 25:24\\]
Determines if the controller can re-order write commands from the same source ID and/or the same port. Bit \\[0\\]
controls source ID usage and bit \\[1\\]
controls port ID usage. Set each bit to 1 to enable usage in placement logic."]
pub type WrOrderReqW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Defines the byte location of byte0 in the memory datapath for device 1 on chip 1. Used for MRRs to identify where data will be returned. DEV=1"]
    #[inline(always)]
    pub fn device1_byte0_cs1(&self) -> Device1Byte0Cs1R {
        Device1Byte0Cs1R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Quantity that determines when the command queue almost full signal will assert \\[q_almost_full\\]. When cleared to 0, the q_almost_full signal will be driven to 0 irrespective of number of entries in the command queue."]
    #[inline(always)]
    pub fn q_fullness(&self) -> QFullnessR {
        QFullnessR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Forces the controller to accept commands in the order in which they are placed in the command queue."]
    #[inline(always)]
    pub fn in_order_accept(&self) -> InOrderAcceptR {
        InOrderAcceptR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Determines if the controller can re-order write commands from the same source ID and/or the same port. Bit \\[0\\]
controls source ID usage and bit \\[1\\]
controls port ID usage. Set each bit to 1 to enable usage in placement logic."]
    #[inline(always)]
    pub fn wr_order_req(&self) -> WrOrderReqR {
        WrOrderReqR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Defines the byte location of byte0 in the memory datapath for device 1 on chip 1. Used for MRRs to identify where data will be returned. DEV=1"]
    #[inline(always)]
    #[must_use]
    pub fn device1_byte0_cs1(
        &mut self,
    ) -> Device1Byte0Cs1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl329Spec> {
        Device1Byte0Cs1W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Quantity that determines when the command queue almost full signal will assert \\[q_almost_full\\]. When cleared to 0, the q_almost_full signal will be driven to 0 irrespective of number of entries in the command queue."]
    #[inline(always)]
    #[must_use]
    pub fn q_fullness(&mut self) -> QFullnessW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl329Spec> {
        QFullnessW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Forces the controller to accept commands in the order in which they are placed in the command queue."]
    #[inline(always)]
    #[must_use]
    pub fn in_order_accept(&mut self) -> InOrderAcceptW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl329Spec> {
        InOrderAcceptW::new(self, 16)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Determines if the controller can re-order write commands from the same source ID and/or the same port. Bit \\[0\\]
controls source ID usage and bit \\[1\\]
controls port ID usage. Set each bit to 1 to enable usage in placement logic."]
    #[inline(always)]
    #[must_use]
    pub fn wr_order_req(&mut self) -> WrOrderReqW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl329Spec> {
        WrOrderReqW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_329\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_329::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_329::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl329Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl329Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_329::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl329Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_329::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl329Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_329 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl329Spec {
    const RESET_VALUE: u32 = 0;
}
