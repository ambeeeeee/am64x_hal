#[doc = "Register `CFG_DCCGCTRL2` reader"]
pub type R = crate::R<CfgDccgctrl2Spec>;
#[doc = "Register `CFG_DCCGCTRL2` writer"]
pub type W = crate::W<CfgDccgctrl2Spec>;
#[doc = "Field `CONT_ON_ERR` reader - 3:0\\]
Continues to next window of comparison despite the error condition. User, privilege, and debug mode (read): Returns the current field value. Privilege and debug mode (write): Sets the value of field value. Enable values: 0101: Comparison and counter reload is stopped from advancing if error is detected. Others: Counters get reloaded with seed and continue counting despite the error condition."]
pub type ContOnErrR = crate::FieldReader;
#[doc = "Field `CONT_ON_ERR` writer - 3:0\\]
Continues to next window of comparison despite the error condition. User, privilege, and debug mode (read): Returns the current field value. Privilege and debug mode (write): Sets the value of field value. Enable values: 0101: Comparison and counter reload is stopped from advancing if error is detected. Others: Counters get reloaded with seed and continue counting despite the error condition."]
pub type ContOnErrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FIFO_READ` reader - 7:4\\]
Enables the counter read registers reflect FIFO output instead of the live counter value. User, privilege, and debug mode (read): Returns the current field value. Privilege and debug mode (write): Sets the value of field value. Source values: 0101: Counter value is read directly. Others: Counters FIFO output is read."]
pub type FifoReadR = crate::FieldReader;
#[doc = "Field `FIFO_READ` writer - 7:4\\]
Enables the counter read registers reflect FIFO output instead of the live counter value. User, privilege, and debug mode (read): Returns the current field value. Privilege and debug mode (write): Sets the value of field value. Source values: 0101: Counter value is read directly. Others: Counters FIFO output is read."]
pub type FifoReadW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FIFO_NONERR` reader - 11:8\\]
Enables/disables FIFO writes without the error event on completion of comparison window. User, privilege, and debug mode (read): Returns the current field value. Privilege and debug mode (write): Sets the value of field value. Source values: 0101: Counter values are captured to non-full FIFO only upon Error event. Others: Write counter values to non-full FIFO upon completion of comparison window regardless of error or not."]
pub type FifoNonerrR = crate::FieldReader;
#[doc = "Field `FIFO_NONERR` writer - 11:8\\]
Enables/disables FIFO writes without the error event on completion of comparison window. User, privilege, and debug mode (read): Returns the current field value. Privilege and debug mode (write): Sets the value of field value. Source values: 0101: Counter values are captured to non-full FIFO only upon Error event. Others: Write counter values to non-full FIFO upon completion of comparison window regardless of error or not."]
pub type FifoNonerrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Continues to next window of comparison despite the error condition. User, privilege, and debug mode (read): Returns the current field value. Privilege and debug mode (write): Sets the value of field value. Enable values: 0101: Comparison and counter reload is stopped from advancing if error is detected. Others: Counters get reloaded with seed and continue counting despite the error condition."]
    #[inline(always)]
    pub fn cont_on_err(&self) -> ContOnErrR {
        ContOnErrR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Enables the counter read registers reflect FIFO output instead of the live counter value. User, privilege, and debug mode (read): Returns the current field value. Privilege and debug mode (write): Sets the value of field value. Source values: 0101: Counter value is read directly. Others: Counters FIFO output is read."]
    #[inline(always)]
    pub fn fifo_read(&self) -> FifoReadR {
        FifoReadR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Enables/disables FIFO writes without the error event on completion of comparison window. User, privilege, and debug mode (read): Returns the current field value. Privilege and debug mode (write): Sets the value of field value. Source values: 0101: Counter values are captured to non-full FIFO only upon Error event. Others: Write counter values to non-full FIFO upon completion of comparison window regardless of error or not."]
    #[inline(always)]
    pub fn fifo_nonerr(&self) -> FifoNonerrR {
        FifoNonerrR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Continues to next window of comparison despite the error condition. User, privilege, and debug mode (read): Returns the current field value. Privilege and debug mode (write): Sets the value of field value. Enable values: 0101: Comparison and counter reload is stopped from advancing if error is detected. Others: Counters get reloaded with seed and continue counting despite the error condition."]
    #[inline(always)]
    #[must_use]
    pub fn cont_on_err(&mut self) -> ContOnErrW<CfgDccgctrl2Spec> {
        ContOnErrW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Enables the counter read registers reflect FIFO output instead of the live counter value. User, privilege, and debug mode (read): Returns the current field value. Privilege and debug mode (write): Sets the value of field value. Source values: 0101: Counter value is read directly. Others: Counters FIFO output is read."]
    #[inline(always)]
    #[must_use]
    pub fn fifo_read(&mut self) -> FifoReadW<CfgDccgctrl2Spec> {
        FifoReadW::new(self, 4)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Enables/disables FIFO writes without the error event on completion of comparison window. User, privilege, and debug mode (read): Returns the current field value. Privilege and debug mode (write): Sets the value of field value. Source values: 0101: Counter values are captured to non-full FIFO only upon Error event. Others: Write counter values to non-full FIFO upon completion of comparison window regardless of error or not."]
    #[inline(always)]
    #[must_use]
    pub fn fifo_nonerr(&mut self) -> FifoNonerrW<CfgDccgctrl2Spec> {
        FifoNonerrW::new(self, 8)
    }
}
#[doc = "Allows configuring different modes of operation for DCC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_dccgctrl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_dccgctrl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgDccgctrl2Spec;
impl crate::RegisterSpec for CfgDccgctrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_dccgctrl2::R`](R) reader structure"]
impl crate::Readable for CfgDccgctrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_dccgctrl2::W`](W) writer structure"]
impl crate::Writable for CfgDccgctrl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_DCCGCTRL2 to value 0x0555"]
impl crate::Resettable for CfgDccgctrl2Spec {
    const RESET_VALUE: u32 = 0x0555;
}
