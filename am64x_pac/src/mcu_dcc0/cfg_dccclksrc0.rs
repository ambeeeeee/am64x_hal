#[doc = "Register `CFG_DCCCLKSRC0` reader"]
pub type R = crate::R<CfgDccclksrc0Spec>;
#[doc = "Register `CFG_DCCCLKSRC0` writer"]
pub type W = crate::W<CfgDccclksrc0Spec>;
#[doc = "Field `CLKSRC0` reader - 3:0\\]
This field specifies the clock source for counter 0. User, privilege, and debug mode (read): Returns the current value of CLKSRC0. Privilege and debug mode (write): Sets the value of CLKSRC0."]
pub type Clksrc0R = crate::FieldReader;
#[doc = "Field `CLKSRC0` writer - 3:0\\]
This field specifies the clock source for counter 0. User, privilege, and debug mode (read): Returns the current value of CLKSRC0. Privilege and debug mode (write): Sets the value of CLKSRC0."]
pub type Clksrc0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `KEY` reader - 15:12\\]
This field enables or disables clock source selection for counter 0. User, privilege, and debug mode (read): Returns the current value of the key. Privilege and debug mode (write): Sets the key value. Key values: 1010: The CLKSRC field selects the clock source for counter 0. others: Clock source selection is disabled. The external oscillator (XTAL) is selected for counter 0."]
pub type KeyR = crate::FieldReader;
#[doc = "Field `KEY` writer - 15:12\\]
This field enables or disables clock source selection for counter 0. User, privilege, and debug mode (read): Returns the current value of the key. Privilege and debug mode (write): Sets the key value. Key values: 1010: The CLKSRC field selects the clock source for counter 0. others: Clock source selection is disabled. The external oscillator (XTAL) is selected for counter 0."]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
This field specifies the clock source for counter 0. User, privilege, and debug mode (read): Returns the current value of CLKSRC0. Privilege and debug mode (write): Sets the value of CLKSRC0."]
    #[inline(always)]
    pub fn clksrc0(&self) -> Clksrc0R {
        Clksrc0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
This field enables or disables clock source selection for counter 0. User, privilege, and debug mode (read): Returns the current value of the key. Privilege and debug mode (write): Sets the key value. Key values: 1010: The CLKSRC field selects the clock source for counter 0. others: Clock source selection is disabled. The external oscillator (XTAL) is selected for counter 0."]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
This field specifies the clock source for counter 0. User, privilege, and debug mode (read): Returns the current value of CLKSRC0. Privilege and debug mode (write): Sets the value of CLKSRC0."]
    #[inline(always)]
    #[must_use]
    pub fn clksrc0(&mut self) -> Clksrc0W<CfgDccclksrc0Spec> {
        Clksrc0W::new(self, 0)
    }
    #[doc = "Bits 12:15 - 15:12\\]
This field enables or disables clock source selection for counter 0. User, privilege, and debug mode (read): Returns the current value of the key. Privilege and debug mode (write): Sets the key value. Key values: 1010: The CLKSRC field selects the clock source for counter 0. others: Clock source selection is disabled. The external oscillator (XTAL) is selected for counter 0."]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KeyW<CfgDccclksrc0Spec> {
        KeyW::new(self, 12)
    }
}
#[doc = "Selects the clock source for counter 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_dccclksrc0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_dccclksrc0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgDccclksrc0Spec;
impl crate::RegisterSpec for CfgDccclksrc0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_dccclksrc0::R`](R) reader structure"]
impl crate::Readable for CfgDccclksrc0Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_dccclksrc0::W`](W) writer structure"]
impl crate::Writable for CfgDccclksrc0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_DCCCLKSRC0 to value 0"]
impl crate::Resettable for CfgDccclksrc0Spec {
    const RESET_VALUE: u32 = 0;
}
