#[doc = "Register `MMR__VBUSP__CFG1_VTM_DEVINFO_PWR0` reader"]
pub type R = crate::R<Mmr_Vbusp_Cfg1VtmDevinfoPwr0Spec>;
#[doc = "Register `MMR__VBUSP__CFG1_VTM_DEVINFO_PWR0` writer"]
pub type W = crate::W<Mmr_Vbusp_Cfg1VtmDevinfoPwr0Spec>;
#[doc = "Field `CVD_CT` reader - 3:0\\]
Number of core voltage domains in device. VD0 is always allocated to VD_RTC, if it exists, and VD1 always to VD_WKUP. The maximum number of cVDs in an SOC is 15, 0xF. Reset value is a VTM tieoff, d_device_cvd_ct."]
pub type CvdCtR = crate::FieldReader;
#[doc = "Field `CVD_CT` writer - 3:0\\]
Number of core voltage domains in device. VD0 is always allocated to VD_RTC, if it exists, and VD1 always to VD_WKUP. The maximum number of cVDs in an SOC is 15, 0xF. Reset value is a VTM tieoff, d_device_cvd_ct."]
pub type CvdCtW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TMPSENS_CT` reader - 7:4\\]
Number of temperature sensors associated with this VTM. Valid values are 4'h0 - 4'h8. 0x0: NO temp-sensor associated to this VTM. 0x8: 8 temp-sensors associated to this VTM. 0x9 to 0xF: invalid values. Reset value is a VTM tieoff, d_vtm_tmpsens_ct."]
pub type TmpsensCtR = crate::FieldReader;
#[doc = "Field `TMPSENS_CT` writer - 7:4\\]
Number of temperature sensors associated with this VTM. Valid values are 4'h0 - 4'h8. 0x0: NO temp-sensor associated to this VTM. 0x8: 8 temp-sensors associated to this VTM. 0x9 to 0xF: invalid values. Reset value is a VTM tieoff, d_vtm_tmpsens_ct."]
pub type TmpsensCtW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `VD_RTC` reader - 12:12\\]
RTC voltage domain presence. 0: There is NO VD_RTC in this SOC. 1: There is a VD_RTC in this SOC. Reset value is a VTM tieoff, d_vd_rtc."]
pub type VdRtcR = crate::BitReader;
#[doc = "Field `VD_RTC` writer - 12:12\\]
RTC voltage domain presence. 0: There is NO VD_RTC in this SOC. 1: There is a VD_RTC in this SOC. Reset value is a VTM tieoff, d_vd_rtc."]
pub type VdRtcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTM_VD_MAP` reader - 19:16\\]
Core voltage domain, cVD, global mapping 4-bit code, in the context of this SOC. It shows in which cVD this VTM is instantiated/placed. This field indicates in which core voltage domain, cVD, has been physically placed this VTM. Valid values: 0x0 to 0xE where: 0x0 = VD_RTC, not present is some SOCs, 0x1 = VD_WKUP, 0x2 = VD_MCU, 0x3 = VD_CORE, not present is some SOCs, 0x4-0xE = Mapping varies between SOCs, 0xF = not implemented. Reset value is a VTM tieoff, d_vtm_vd_map."]
pub type VtmVdMapR = crate::FieldReader;
#[doc = "Field `VTM_VD_MAP` writer - 19:16\\]
Core voltage domain, cVD, global mapping 4-bit code, in the context of this SOC. It shows in which cVD this VTM is instantiated/placed. This field indicates in which core voltage domain, cVD, has been physically placed this VTM. Valid values: 0x0 to 0xE where: 0x0 = VD_RTC, not present is some SOCs, 0x1 = VD_WKUP, 0x2 = VD_MCU, 0x3 = VD_CORE, not present is some SOCs, 0x4-0xE = Mapping varies between SOCs, 0xF = not implemented. Reset value is a VTM tieoff, d_vtm_vd_map."]
pub type VtmVdMapW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Number of core voltage domains in device. VD0 is always allocated to VD_RTC, if it exists, and VD1 always to VD_WKUP. The maximum number of cVDs in an SOC is 15, 0xF. Reset value is a VTM tieoff, d_device_cvd_ct."]
    #[inline(always)]
    pub fn cvd_ct(&self) -> CvdCtR {
        CvdCtR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Number of temperature sensors associated with this VTM. Valid values are 4'h0 - 4'h8. 0x0: NO temp-sensor associated to this VTM. 0x8: 8 temp-sensors associated to this VTM. 0x9 to 0xF: invalid values. Reset value is a VTM tieoff, d_vtm_tmpsens_ct."]
    #[inline(always)]
    pub fn tmpsens_ct(&self) -> TmpsensCtR {
        TmpsensCtR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - 12:12\\]
RTC voltage domain presence. 0: There is NO VD_RTC in this SOC. 1: There is a VD_RTC in this SOC. Reset value is a VTM tieoff, d_vd_rtc."]
    #[inline(always)]
    pub fn vd_rtc(&self) -> VdRtcR {
        VdRtcR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Core voltage domain, cVD, global mapping 4-bit code, in the context of this SOC. It shows in which cVD this VTM is instantiated/placed. This field indicates in which core voltage domain, cVD, has been physically placed this VTM. Valid values: 0x0 to 0xE where: 0x0 = VD_RTC, not present is some SOCs, 0x1 = VD_WKUP, 0x2 = VD_MCU, 0x3 = VD_CORE, not present is some SOCs, 0x4-0xE = Mapping varies between SOCs, 0xF = not implemented. Reset value is a VTM tieoff, d_vtm_vd_map."]
    #[inline(always)]
    pub fn vtm_vd_map(&self) -> VtmVdMapR {
        VtmVdMapR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Number of core voltage domains in device. VD0 is always allocated to VD_RTC, if it exists, and VD1 always to VD_WKUP. The maximum number of cVDs in an SOC is 15, 0xF. Reset value is a VTM tieoff, d_device_cvd_ct."]
    #[inline(always)]
    #[must_use]
    pub fn cvd_ct(&mut self) -> CvdCtW<Mmr_Vbusp_Cfg1VtmDevinfoPwr0Spec> {
        CvdCtW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Number of temperature sensors associated with this VTM. Valid values are 4'h0 - 4'h8. 0x0: NO temp-sensor associated to this VTM. 0x8: 8 temp-sensors associated to this VTM. 0x9 to 0xF: invalid values. Reset value is a VTM tieoff, d_vtm_tmpsens_ct."]
    #[inline(always)]
    #[must_use]
    pub fn tmpsens_ct(&mut self) -> TmpsensCtW<Mmr_Vbusp_Cfg1VtmDevinfoPwr0Spec> {
        TmpsensCtW::new(self, 4)
    }
    #[doc = "Bit 12 - 12:12\\]
RTC voltage domain presence. 0: There is NO VD_RTC in this SOC. 1: There is a VD_RTC in this SOC. Reset value is a VTM tieoff, d_vd_rtc."]
    #[inline(always)]
    #[must_use]
    pub fn vd_rtc(&mut self) -> VdRtcW<Mmr_Vbusp_Cfg1VtmDevinfoPwr0Spec> {
        VdRtcW::new(self, 12)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Core voltage domain, cVD, global mapping 4-bit code, in the context of this SOC. It shows in which cVD this VTM is instantiated/placed. This field indicates in which core voltage domain, cVD, has been physically placed this VTM. Valid values: 0x0 to 0xE where: 0x0 = VD_RTC, not present is some SOCs, 0x1 = VD_WKUP, 0x2 = VD_MCU, 0x3 = VD_CORE, not present is some SOCs, 0x4-0xE = Mapping varies between SOCs, 0xF = not implemented. Reset value is a VTM tieoff, d_vtm_vd_map."]
    #[inline(always)]
    #[must_use]
    pub fn vtm_vd_map(&mut self) -> VtmVdMapW<Mmr_Vbusp_Cfg1VtmDevinfoPwr0Spec> {
        VtmVdMapW::new(self, 16)
    }
}
#[doc = "Device specific voltage domain and temp sensor information register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__vbusp__cfg1_vtm_devinfo_pwr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__vbusp__cfg1_vtm_devinfo_pwr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mmr_Vbusp_Cfg1VtmDevinfoPwr0Spec;
impl crate::RegisterSpec for Mmr_Vbusp_Cfg1VtmDevinfoPwr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmr__vbusp__cfg1_vtm_devinfo_pwr0::R`](R) reader structure"]
impl crate::Readable for Mmr_Vbusp_Cfg1VtmDevinfoPwr0Spec {}
#[doc = "`write(|w| ..)` method takes [`mmr__vbusp__cfg1_vtm_devinfo_pwr0::W`](W) writer structure"]
impl crate::Writable for Mmr_Vbusp_Cfg1VtmDevinfoPwr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMR__VBUSP__CFG1_VTM_DEVINFO_PWR0 to value 0"]
impl crate::Resettable for Mmr_Vbusp_Cfg1VtmDevinfoPwr0Spec {
    const RESET_VALUE: u32 = 0;
}
