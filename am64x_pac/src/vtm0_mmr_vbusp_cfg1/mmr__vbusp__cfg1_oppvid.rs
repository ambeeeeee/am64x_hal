#[doc = "Register `MMR__VBUSP__CFG1_OPPVID` reader"]
pub type R = crate::R<Mmr_Vbusp_Cfg1OppvidSpec>;
#[doc = "Register `MMR__VBUSP__CFG1_OPPVID` writer"]
pub type W = crate::W<Mmr_Vbusp_Cfg1OppvidSpec>;
#[doc = "Field `OPP_0` reader - 7:0\\]
OPP 0 default VID. VID code that represents the required VD value in this Voltage domain to operate at. Valid values are from 0x1e to 0x91. Any value outside that range indicates not implemented including 0x0. This is SOC and device/chip specific. Reset value is from e-fuse at POR, efuse_vd\\[a\\]_opp_0."]
pub type Opp0R = crate::FieldReader;
#[doc = "Field `OPP_0` writer - 7:0\\]
OPP 0 default VID. VID code that represents the required VD value in this Voltage domain to operate at. Valid values are from 0x1e to 0x91. Any value outside that range indicates not implemented including 0x0. This is SOC and device/chip specific. Reset value is from e-fuse at POR, efuse_vd\\[a\\]_opp_0."]
pub type Opp0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `OPP_1` reader - 15:8\\]
OPP 1 default VID. VID code that represents the required VD value in this Voltage domain to operate at. Valid values are from 0x1e to 0x91. Any value outside that range indicates not implemented including 0x0. This is SOC and device/chip specific. Reset value is from e-fuse at POR, efuse_vd\\[a\\]_opp_1."]
pub type Opp1R = crate::FieldReader;
#[doc = "Field `OPP_1` writer - 15:8\\]
OPP 1 default VID. VID code that represents the required VD value in this Voltage domain to operate at. Valid values are from 0x1e to 0x91. Any value outside that range indicates not implemented including 0x0. This is SOC and device/chip specific. Reset value is from e-fuse at POR, efuse_vd\\[a\\]_opp_1."]
pub type Opp1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `OPP_2` reader - 23:16\\]
OPP 2 default VID. VID code that represents the required VD value in this Voltage domain to operate at. Valid values are from 0x1e to 0x91. Any value outside that range indicates not implemented including 0x0. This is SOC and device/chip specific. Reset value is from e-fuse at POR, efuse_vd\\[a\\]_opp_2."]
pub type Opp2R = crate::FieldReader;
#[doc = "Field `OPP_2` writer - 23:16\\]
OPP 2 default VID. VID code that represents the required VD value in this Voltage domain to operate at. Valid values are from 0x1e to 0x91. Any value outside that range indicates not implemented including 0x0. This is SOC and device/chip specific. Reset value is from e-fuse at POR, efuse_vd\\[a\\]_opp_2."]
pub type Opp2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `OPP_3` reader - 31:24\\]
OPP 3 default VID. VID code that represents the required VD value in this Voltage domain to operate at. Valid values are from 0x1e to 0x91. Any value outside that range indicates not implemented including 0x0. This is SOC and device/chip specific. Reset value is from e-fuse at POR, efuse_vd\\[a\\]_opp_3."]
pub type Opp3R = crate::FieldReader;
#[doc = "Field `OPP_3` writer - 31:24\\]
OPP 3 default VID. VID code that represents the required VD value in this Voltage domain to operate at. Valid values are from 0x1e to 0x91. Any value outside that range indicates not implemented including 0x0. This is SOC and device/chip specific. Reset value is from e-fuse at POR, efuse_vd\\[a\\]_opp_3."]
pub type Opp3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
OPP 0 default VID. VID code that represents the required VD value in this Voltage domain to operate at. Valid values are from 0x1e to 0x91. Any value outside that range indicates not implemented including 0x0. This is SOC and device/chip specific. Reset value is from e-fuse at POR, efuse_vd\\[a\\]_opp_0."]
    #[inline(always)]
    pub fn opp_0(&self) -> Opp0R {
        Opp0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
OPP 1 default VID. VID code that represents the required VD value in this Voltage domain to operate at. Valid values are from 0x1e to 0x91. Any value outside that range indicates not implemented including 0x0. This is SOC and device/chip specific. Reset value is from e-fuse at POR, efuse_vd\\[a\\]_opp_1."]
    #[inline(always)]
    pub fn opp_1(&self) -> Opp1R {
        Opp1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
OPP 2 default VID. VID code that represents the required VD value in this Voltage domain to operate at. Valid values are from 0x1e to 0x91. Any value outside that range indicates not implemented including 0x0. This is SOC and device/chip specific. Reset value is from e-fuse at POR, efuse_vd\\[a\\]_opp_2."]
    #[inline(always)]
    pub fn opp_2(&self) -> Opp2R {
        Opp2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
OPP 3 default VID. VID code that represents the required VD value in this Voltage domain to operate at. Valid values are from 0x1e to 0x91. Any value outside that range indicates not implemented including 0x0. This is SOC and device/chip specific. Reset value is from e-fuse at POR, efuse_vd\\[a\\]_opp_3."]
    #[inline(always)]
    pub fn opp_3(&self) -> Opp3R {
        Opp3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
OPP 0 default VID. VID code that represents the required VD value in this Voltage domain to operate at. Valid values are from 0x1e to 0x91. Any value outside that range indicates not implemented including 0x0. This is SOC and device/chip specific. Reset value is from e-fuse at POR, efuse_vd\\[a\\]_opp_0."]
    #[inline(always)]
    #[must_use]
    pub fn opp_0(&mut self) -> Opp0W<Mmr_Vbusp_Cfg1OppvidSpec> {
        Opp0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
OPP 1 default VID. VID code that represents the required VD value in this Voltage domain to operate at. Valid values are from 0x1e to 0x91. Any value outside that range indicates not implemented including 0x0. This is SOC and device/chip specific. Reset value is from e-fuse at POR, efuse_vd\\[a\\]_opp_1."]
    #[inline(always)]
    #[must_use]
    pub fn opp_1(&mut self) -> Opp1W<Mmr_Vbusp_Cfg1OppvidSpec> {
        Opp1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
OPP 2 default VID. VID code that represents the required VD value in this Voltage domain to operate at. Valid values are from 0x1e to 0x91. Any value outside that range indicates not implemented including 0x0. This is SOC and device/chip specific. Reset value is from e-fuse at POR, efuse_vd\\[a\\]_opp_2."]
    #[inline(always)]
    #[must_use]
    pub fn opp_2(&mut self) -> Opp2W<Mmr_Vbusp_Cfg1OppvidSpec> {
        Opp2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
OPP 3 default VID. VID code that represents the required VD value in this Voltage domain to operate at. Valid values are from 0x1e to 0x91. Any value outside that range indicates not implemented including 0x0. This is SOC and device/chip specific. Reset value is from e-fuse at POR, efuse_vd\\[a\\]_opp_3."]
    #[inline(always)]
    #[must_use]
    pub fn opp_3(&mut self) -> Opp3W<Mmr_Vbusp_Cfg1OppvidSpec> {
        Opp3W::new(self, 24)
    }
}
#[doc = "Voltage domain a VID actual code used as reference by Firmware to set the various voltage domain supply voltages. Reset defaults are sourced from efuse for each OPP. The default reset values will not be necessarily overwritten. The write capability in the MMR is for having the option to debug and have software driven adjustments if necessary.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__vbusp__cfg1_oppvid::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__vbusp__cfg1_oppvid::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mmr_Vbusp_Cfg1OppvidSpec;
impl crate::RegisterSpec for Mmr_Vbusp_Cfg1OppvidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmr__vbusp__cfg1_oppvid::R`](R) reader structure"]
impl crate::Readable for Mmr_Vbusp_Cfg1OppvidSpec {}
#[doc = "`write(|w| ..)` method takes [`mmr__vbusp__cfg1_oppvid::W`](W) writer structure"]
impl crate::Writable for Mmr_Vbusp_Cfg1OppvidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMR__VBUSP__CFG1_OPPVID to value 0"]
impl crate::Resettable for Mmr_Vbusp_Cfg1OppvidSpec {
    const RESET_VALUE: u32 = 0;
}
