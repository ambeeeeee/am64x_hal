#[doc = "Register `REGS__SS_CFG__SSCFG_SS_ID_REV_REG` reader"]
pub type R = crate::R<Regs_SsCfg_SscfgSsIdRevRegSpec>;
#[doc = "Register `REGS__SS_CFG__SSCFG_SS_ID_REV_REG` writer"]
pub type W = crate::W<Regs_SsCfg_SscfgSsIdRevRegSpec>;
#[doc = "Field `MIN_REV` reader - 5:0\\]
Minor revision"]
pub type MinRevR = crate::FieldReader;
#[doc = "Field `MIN_REV` writer - 5:0\\]
Minor revision"]
pub type MinRevW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CUSTOM` reader - 7:6\\]
Custom"]
pub type CustomR = crate::FieldReader;
#[doc = "Field `CUSTOM` writer - 7:6\\]
Custom"]
pub type CustomW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MAJ_REV` reader - 10:8\\]
Major revision"]
pub type MajRevR = crate::FieldReader;
#[doc = "Field `MAJ_REV` writer - 10:8\\]
Major revision"]
pub type MajRevW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RTL_VER` reader - 15:11\\]
RTL version"]
pub type RtlVerR = crate::FieldReader;
#[doc = "Field `RTL_VER` writer - 15:11\\]
RTL version"]
pub type RtlVerW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `MOD_ID` reader - 31:16\\]
Module ID"]
pub type ModIdR = crate::FieldReader<u16>;
#[doc = "Field `MOD_ID` writer - 31:16\\]
Module ID"]
pub type ModIdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Minor revision"]
    #[inline(always)]
    pub fn min_rev(&self) -> MinRevR {
        MinRevR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Custom"]
    #[inline(always)]
    pub fn custom(&self) -> CustomR {
        CustomR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Major revision"]
    #[inline(always)]
    pub fn maj_rev(&self) -> MajRevR {
        MajRevR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
RTL version"]
    #[inline(always)]
    pub fn rtl_ver(&self) -> RtlVerR {
        RtlVerR::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Module ID"]
    #[inline(always)]
    pub fn mod_id(&self) -> ModIdR {
        ModIdR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Minor revision"]
    #[inline(always)]
    #[must_use]
    pub fn min_rev(&mut self) -> MinRevW<Regs_SsCfg_SscfgSsIdRevRegSpec> {
        MinRevW::new(self, 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Custom"]
    #[inline(always)]
    #[must_use]
    pub fn custom(&mut self) -> CustomW<Regs_SsCfg_SscfgSsIdRevRegSpec> {
        CustomW::new(self, 6)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Major revision"]
    #[inline(always)]
    #[must_use]
    pub fn maj_rev(&mut self) -> MajRevW<Regs_SsCfg_SscfgSsIdRevRegSpec> {
        MajRevW::new(self, 8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
RTL version"]
    #[inline(always)]
    #[must_use]
    pub fn rtl_ver(&mut self) -> RtlVerW<Regs_SsCfg_SscfgSsIdRevRegSpec> {
        RtlVerW::new(self, 11)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Module ID"]
    #[inline(always)]
    #[must_use]
    pub fn mod_id(&mut self) -> ModIdW<Regs_SsCfg_SscfgSsIdRevRegSpec> {
        ModIdW::new(self, 16)
    }
}
#[doc = "The Subsystem ID and Revision Register contains the module ID, major, and minor revisions for the subsystem\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_ss_id_rev_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_ss_id_rev_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regs_SsCfg_SscfgSsIdRevRegSpec;
impl crate::RegisterSpec for Regs_SsCfg_SscfgSsIdRevRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs__ss_cfg__sscfg_ss_id_rev_reg::R`](R) reader structure"]
impl crate::Readable for Regs_SsCfg_SscfgSsIdRevRegSpec {}
#[doc = "`write(|w| ..)` method takes [`regs__ss_cfg__sscfg_ss_id_rev_reg::W`](W) writer structure"]
impl crate::Writable for Regs_SsCfg_SscfgSsIdRevRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS__SS_CFG__SSCFG_SS_ID_REV_REG to value 0x3200"]
impl crate::Resettable for Regs_SsCfg_SscfgSsIdRevRegSpec {
    const RESET_VALUE: u32 = 0x3200;
}
