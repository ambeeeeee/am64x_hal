#[doc = "Register `CONFIG_TIMEOUT_STATUS1` reader"]
pub type R = crate::R<ConfigTimeoutStatus1Spec>;
#[doc = "Register `CONFIG_TIMEOUT_STATUS1` writer"]
pub type W = crate::W<ConfigTimeoutStatus1Spec>;
#[doc = "Field `EXPIRED_TIMER1` reader - 10:0\\]
The ID of the second timer to expire"]
pub type ExpiredTimer1R = crate::FieldReader<u16>;
#[doc = "Field `EXPIRED_TIMER1` writer - 10:0\\]
The ID of the second timer to expire"]
pub type ExpiredTimer1W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `VALID1` reader - 11:11\\]
1 indicates that expired_timer1 is valid"]
pub type Valid1R = crate::BitReader;
#[doc = "Field `VALID1` writer - 11:11\\]
1 indicates that expired_timer1 is valid"]
pub type Valid1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXPIRED_TIMER2` reader - 22:12\\]
The ID of the third timer to expire"]
pub type ExpiredTimer2R = crate::FieldReader<u16>;
#[doc = "Field `EXPIRED_TIMER2` writer - 22:12\\]
The ID of the third timer to expire"]
pub type ExpiredTimer2W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `VALID2` reader - 23:23\\]
1 indicates that expired_timer2 is valid"]
pub type Valid2R = crate::BitReader;
#[doc = "Field `VALID2` writer - 23:23\\]
1 indicates that expired_timer2 is valid"]
pub type Valid2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - 10:0\\]
The ID of the second timer to expire"]
    #[inline(always)]
    pub fn expired_timer1(&self) -> ExpiredTimer1R {
        ExpiredTimer1R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 11 - 11:11\\]
1 indicates that expired_timer1 is valid"]
    #[inline(always)]
    pub fn valid1(&self) -> Valid1R {
        Valid1R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:22 - 22:12\\]
The ID of the third timer to expire"]
    #[inline(always)]
    pub fn expired_timer2(&self) -> ExpiredTimer2R {
        ExpiredTimer2R::new(((self.bits >> 12) & 0x07ff) as u16)
    }
    #[doc = "Bit 23 - 23:23\\]
1 indicates that expired_timer2 is valid"]
    #[inline(always)]
    pub fn valid2(&self) -> Valid2R {
        Valid2R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - 10:0\\]
The ID of the second timer to expire"]
    #[inline(always)]
    #[must_use]
    pub fn expired_timer1(&mut self) -> ExpiredTimer1W<ConfigTimeoutStatus1Spec> {
        ExpiredTimer1W::new(self, 0)
    }
    #[doc = "Bit 11 - 11:11\\]
1 indicates that expired_timer1 is valid"]
    #[inline(always)]
    #[must_use]
    pub fn valid1(&mut self) -> Valid1W<ConfigTimeoutStatus1Spec> {
        Valid1W::new(self, 11)
    }
    #[doc = "Bits 12:22 - 22:12\\]
The ID of the third timer to expire"]
    #[inline(always)]
    #[must_use]
    pub fn expired_timer2(&mut self) -> ExpiredTimer2W<ConfigTimeoutStatus1Spec> {
        ExpiredTimer2W::new(self, 12)
    }
    #[doc = "Bit 23 - 23:23\\]
1 indicates that expired_timer2 is valid"]
    #[inline(always)]
    #[must_use]
    pub fn valid2(&mut self) -> Valid2W<ConfigTimeoutStatus1Spec> {
        Valid2W::new(self, 23)
    }
}
#[doc = "This register contains the IDs of the second and third timers to expire. It is indended as a more efficient way of finding the first few timers to expire rather than needing to read the status of all 1024 timers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config_timeout_status1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config_timeout_status1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigTimeoutStatus1Spec;
impl crate::RegisterSpec for ConfigTimeoutStatus1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config_timeout_status1::R`](R) reader structure"]
impl crate::Readable for ConfigTimeoutStatus1Spec {}
#[doc = "`write(|w| ..)` method takes [`config_timeout_status1::W`](W) writer structure"]
impl crate::Writable for ConfigTimeoutStatus1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONFIG_TIMEOUT_STATUS1 to value 0"]
impl crate::Resettable for ConfigTimeoutStatus1Spec {
    const RESET_VALUE: u32 = 0;
}
