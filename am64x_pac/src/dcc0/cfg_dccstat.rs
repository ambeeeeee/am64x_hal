#[doc = "Register `CFG_DCCSTAT` reader"]
pub type R = crate::R<CfgDccstatSpec>;
#[doc = "Register `CFG_DCCSTAT` writer"]
pub type W = crate::W<CfgDccstatSpec>;
#[doc = "Field `ERRFLG` reader - 0:0\\]
Indicates whether or not an error has occured. Writing a 1 to this bit clears the flag. User, privilege, and debug mode (read): 0 = an error has not occurred 1 = an error has occurred Privilege and debug mode (write): 0 = no effect 1 = clear the error flag"]
pub type ErrflgR = crate::BitReader;
#[doc = "Field `ERRFLG` writer - 0:0\\]
Indicates whether or not an error has occured. Writing a 1 to this bit clears the flag. User, privilege, and debug mode (read): 0 = an error has not occurred 1 = an error has occurred Privilege and debug mode (write): 0 = no effect 1 = clear the error flag"]
pub type ErrflgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DONEFLG` reader - 1:1\\]
Indicates when single-shot mode is complete without error. Writing a 1 to this bit clears the flag. User, privilege, and debug mode (read): 0 = single-shot mode is not done 1 = single-shot mode is done Privilege and debug mode (write): 0 = no effect 1 = clear the done flag"]
pub type DoneflgR = crate::BitReader;
#[doc = "Field `DONEFLG` writer - 1:1\\]
Indicates when single-shot mode is complete without error. Writing a 1 to this bit clears the flag. User, privilege, and debug mode (read): 0 = single-shot mode is not done 1 = single-shot mode is done Privilege and debug mode (write): 0 = no effect 1 = clear the done flag"]
pub type DoneflgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Indicates whether or not an error has occured. Writing a 1 to this bit clears the flag. User, privilege, and debug mode (read): 0 = an error has not occurred 1 = an error has occurred Privilege and debug mode (write): 0 = no effect 1 = clear the error flag"]
    #[inline(always)]
    pub fn errflg(&self) -> ErrflgR {
        ErrflgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Indicates when single-shot mode is complete without error. Writing a 1 to this bit clears the flag. User, privilege, and debug mode (read): 0 = single-shot mode is not done 1 = single-shot mode is done Privilege and debug mode (write): 0 = no effect 1 = clear the done flag"]
    #[inline(always)]
    pub fn doneflg(&self) -> DoneflgR {
        DoneflgR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Indicates whether or not an error has occured. Writing a 1 to this bit clears the flag. User, privilege, and debug mode (read): 0 = an error has not occurred 1 = an error has occurred Privilege and debug mode (write): 0 = no effect 1 = clear the error flag"]
    #[inline(always)]
    #[must_use]
    pub fn errflg(&mut self) -> ErrflgW<CfgDccstatSpec> {
        ErrflgW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Indicates when single-shot mode is complete without error. Writing a 1 to this bit clears the flag. User, privilege, and debug mode (read): 0 = single-shot mode is not done 1 = single-shot mode is done Privilege and debug mode (write): 0 = no effect 1 = clear the done flag"]
    #[inline(always)]
    #[must_use]
    pub fn doneflg(&mut self) -> DoneflgW<CfgDccstatSpec> {
        DoneflgW::new(self, 1)
    }
}
#[doc = "Specifies the status of the DCC Module.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_dccstat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_dccstat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgDccstatSpec;
impl crate::RegisterSpec for CfgDccstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_dccstat::R`](R) reader structure"]
impl crate::Readable for CfgDccstatSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_dccstat::W`](W) writer structure"]
impl crate::Writable for CfgDccstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_DCCSTAT to value 0"]
impl crate::Resettable for CfgDccstatSpec {
    const RESET_VALUE: u32 = 0;
}
