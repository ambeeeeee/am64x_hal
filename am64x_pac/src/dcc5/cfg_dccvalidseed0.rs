#[doc = "Register `CFG_DCCVALIDSEED0` reader"]
pub type R = crate::R<CfgDccvalidseed0Spec>;
#[doc = "Register `CFG_DCCVALIDSEED0` writer"]
pub type W = crate::W<CfgDccvalidseed0Spec>;
#[doc = "Field `VALIDSEED0` reader - 15:0\\]
This field contains the seed value that gets loaded into the valid duration counter for clock source 0. User, privilege, and debug mode (read): Returns the current seed value for VALID0. Privilege and debug mode (write): Sets the current seed value for VALID0."]
pub type Validseed0R = crate::FieldReader<u16>;
#[doc = "Field `VALIDSEED0` writer - 15:0\\]
This field contains the seed value that gets loaded into the valid duration counter for clock source 0. User, privilege, and debug mode (read): Returns the current seed value for VALID0. Privilege and debug mode (write): Sets the current seed value for VALID0."]
pub type Validseed0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
This field contains the seed value that gets loaded into the valid duration counter for clock source 0. User, privilege, and debug mode (read): Returns the current seed value for VALID0. Privilege and debug mode (write): Sets the current seed value for VALID0."]
    #[inline(always)]
    pub fn validseed0(&self) -> Validseed0R {
        Validseed0R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
This field contains the seed value that gets loaded into the valid duration counter for clock source 0. User, privilege, and debug mode (read): Returns the current seed value for VALID0. Privilege and debug mode (write): Sets the current seed value for VALID0."]
    #[inline(always)]
    #[must_use]
    pub fn validseed0(&mut self) -> Validseed0W<CfgDccvalidseed0Spec> {
        Validseed0W::new(self, 0)
    }
}
#[doc = "Seed value for the timeout counter attached to clock source 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_dccvalidseed0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_dccvalidseed0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgDccvalidseed0Spec;
impl crate::RegisterSpec for CfgDccvalidseed0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_dccvalidseed0::R`](R) reader structure"]
impl crate::Readable for CfgDccvalidseed0Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_dccvalidseed0::W`](W) writer structure"]
impl crate::Writable for CfgDccvalidseed0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_DCCVALIDSEED0 to value 0"]
impl crate::Resettable for CfgDccvalidseed0Spec {
    const RESET_VALUE: u32 = 0;
}
