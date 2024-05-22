#[doc = "Register `CTI__CFG__CSCTI_CFG_AUTHSTATUS` reader"]
pub type R = crate::R<Cti_Cfg_CsctiCfgAuthstatusSpec>;
#[doc = "Register `CTI__CFG__CSCTI_CFG_AUTHSTATUS` writer"]
pub type W = crate::W<Cti_Cfg_CsctiCfgAuthstatusSpec>;
#[doc = "Field `NSID` reader - 1:0\\]
Indicates the security level for non-secure invasive debug"]
pub type NsidR = crate::FieldReader;
#[doc = "Field `NSID` writer - 1:0\\]
Indicates the security level for non-secure invasive debug"]
pub type NsidW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NSNID` reader - 3:2\\]
Indicates the security level for non-secure non-invasive debug"]
pub type NsnidR = crate::FieldReader;
#[doc = "Field `NSNID` writer - 3:2\\]
Indicates the security level for non-secure non-invasive debug"]
pub type NsnidW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SID` reader - 5:4\\]
Indicates the security level for secure invasive debug"]
pub type SidR = crate::FieldReader;
#[doc = "Field `SID` writer - 5:4\\]
Indicates the security level for secure invasive debug"]
pub type SidW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SNID` reader - 7:6\\]
Indicates the security level for secure non-invasive debug"]
pub type SnidR = crate::FieldReader;
#[doc = "Field `SNID` writer - 7:6\\]
Indicates the security level for secure non-invasive debug"]
pub type SnidW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Indicates the security level for non-secure invasive debug"]
    #[inline(always)]
    pub fn nsid(&self) -> NsidR {
        NsidR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Indicates the security level for non-secure non-invasive debug"]
    #[inline(always)]
    pub fn nsnid(&self) -> NsnidR {
        NsnidR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Indicates the security level for secure invasive debug"]
    #[inline(always)]
    pub fn sid(&self) -> SidR {
        SidR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Indicates the security level for secure non-invasive debug"]
    #[inline(always)]
    pub fn snid(&self) -> SnidR {
        SnidR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Indicates the security level for non-secure invasive debug"]
    #[inline(always)]
    #[must_use]
    pub fn nsid(&mut self) -> NsidW<Cti_Cfg_CsctiCfgAuthstatusSpec> {
        NsidW::new(self, 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Indicates the security level for non-secure non-invasive debug"]
    #[inline(always)]
    #[must_use]
    pub fn nsnid(&mut self) -> NsnidW<Cti_Cfg_CsctiCfgAuthstatusSpec> {
        NsnidW::new(self, 2)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Indicates the security level for secure invasive debug"]
    #[inline(always)]
    #[must_use]
    pub fn sid(&mut self) -> SidW<Cti_Cfg_CsctiCfgAuthstatusSpec> {
        SidW::new(self, 4)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Indicates the security level for secure non-invasive debug"]
    #[inline(always)]
    #[must_use]
    pub fn snid(&mut self) -> SnidW<Cti_Cfg_CsctiCfgAuthstatusSpec> {
        SnidW::new(self, 6)
    }
}
#[doc = "Reports what functionality is currently permitted by the authentication interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_authstatus::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_authstatus::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cti_Cfg_CsctiCfgAuthstatusSpec;
impl crate::RegisterSpec for Cti_Cfg_CsctiCfgAuthstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cti__cfg__cscti_cfg_authstatus::R`](R) reader structure"]
impl crate::Readable for Cti_Cfg_CsctiCfgAuthstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`cti__cfg__cscti_cfg_authstatus::W`](W) writer structure"]
impl crate::Writable for Cti_Cfg_CsctiCfgAuthstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTI__CFG__CSCTI_CFG_AUTHSTATUS to value 0x0a"]
impl crate::Resettable for Cti_Cfg_CsctiCfgAuthstatusSpec {
    const RESET_VALUE: u32 = 0x0a;
}
