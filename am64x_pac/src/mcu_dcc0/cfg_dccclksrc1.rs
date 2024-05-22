#[doc = "Register `CFG_DCCCLKSRC1` reader"]
pub type R = crate::R<CfgDccclksrc1Spec>;
#[doc = "Register `CFG_DCCCLKSRC1` writer"]
pub type W = crate::W<CfgDccclksrc1Spec>;
#[doc = "Field `CLKSRC1` reader - 4:0\\]
This field specifies the clock source for counter 1, when the KEY field enables this feature. User, privilege, and debug mode (read): Returns the current value of CLKSRC. Privilege and debug mode (write): Sets the value of CLKSRC."]
pub type Clksrc1R = crate::FieldReader;
#[doc = "Field `CLKSRC1` writer - 4:0\\]
This field specifies the clock source for counter 1, when the KEY field enables this feature. User, privilege, and debug mode (read): Returns the current value of CLKSRC. Privilege and debug mode (write): Sets the value of CLKSRC."]
pub type Clksrc1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `KEY` reader - 15:12\\]
This field enables or disables clock source selection for counter 1. User, privilege, and debug mode (read): Returns the current value of the key. Privilege and debug mode (write): Sets the key value. Key values: 1010: The CLKSRC field selects the clock source for counter 1. others: Clock source selection is disabled. The secondary oscillator (clock source 1) is selected for counter 1."]
pub type KeyR = crate::FieldReader;
#[doc = "Field `KEY` writer - 15:12\\]
This field enables or disables clock source selection for counter 1. User, privilege, and debug mode (read): Returns the current value of the key. Privilege and debug mode (write): Sets the key value. Key values: 1010: The CLKSRC field selects the clock source for counter 1. others: Clock source selection is disabled. The secondary oscillator (clock source 1) is selected for counter 1."]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
This field specifies the clock source for counter 1, when the KEY field enables this feature. User, privilege, and debug mode (read): Returns the current value of CLKSRC. Privilege and debug mode (write): Sets the value of CLKSRC."]
    #[inline(always)]
    pub fn clksrc1(&self) -> Clksrc1R {
        Clksrc1R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
This field enables or disables clock source selection for counter 1. User, privilege, and debug mode (read): Returns the current value of the key. Privilege and debug mode (write): Sets the key value. Key values: 1010: The CLKSRC field selects the clock source for counter 1. others: Clock source selection is disabled. The secondary oscillator (clock source 1) is selected for counter 1."]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
This field specifies the clock source for counter 1, when the KEY field enables this feature. User, privilege, and debug mode (read): Returns the current value of CLKSRC. Privilege and debug mode (write): Sets the value of CLKSRC."]
    #[inline(always)]
    #[must_use]
    pub fn clksrc1(&mut self) -> Clksrc1W<CfgDccclksrc1Spec> {
        Clksrc1W::new(self, 0)
    }
    #[doc = "Bits 12:15 - 15:12\\]
This field enables or disables clock source selection for counter 1. User, privilege, and debug mode (read): Returns the current value of the key. Privilege and debug mode (write): Sets the key value. Key values: 1010: The CLKSRC field selects the clock source for counter 1. others: Clock source selection is disabled. The secondary oscillator (clock source 1) is selected for counter 1."]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KeyW<CfgDccclksrc1Spec> {
        KeyW::new(self, 12)
    }
}
#[doc = "Selects the clock source for counter 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_dccclksrc1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_dccclksrc1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgDccclksrc1Spec;
impl crate::RegisterSpec for CfgDccclksrc1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_dccclksrc1::R`](R) reader structure"]
impl crate::Readable for CfgDccclksrc1Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_dccclksrc1::W`](W) writer structure"]
impl crate::Writable for CfgDccclksrc1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_DCCCLKSRC1 to value 0"]
impl crate::Resettable for CfgDccclksrc1Spec {
    const RESET_VALUE: u32 = 0;
}
