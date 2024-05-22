#[doc = "Register `ADCREGS_STEPENABLE` reader"]
pub type R = crate::R<AdcregsStepenableSpec>;
#[doc = "Register `ADCREGS_STEPENABLE` writer"]
pub type W = crate::W<AdcregsStepenableSpec>;
#[doc = "Field `STEP1` reader - 1:1\\]
Enable step"]
pub type Step1R = crate::BitReader;
#[doc = "Field `STEP1` writer - 1:1\\]
Enable step"]
pub type Step1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STEP2` reader - 2:2\\]
Enable step"]
pub type Step2R = crate::BitReader;
#[doc = "Field `STEP2` writer - 2:2\\]
Enable step"]
pub type Step2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STEP3` reader - 3:3\\]
Enable step"]
pub type Step3R = crate::BitReader;
#[doc = "Field `STEP3` writer - 3:3\\]
Enable step"]
pub type Step3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STEP4` reader - 4:4\\]
Enable step"]
pub type Step4R = crate::BitReader;
#[doc = "Field `STEP4` writer - 4:4\\]
Enable step"]
pub type Step4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STEP5` reader - 5:5\\]
Enable step"]
pub type Step5R = crate::BitReader;
#[doc = "Field `STEP5` writer - 5:5\\]
Enable step"]
pub type Step5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STEP6` reader - 6:6\\]
Enable step"]
pub type Step6R = crate::BitReader;
#[doc = "Field `STEP6` writer - 6:6\\]
Enable step"]
pub type Step6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STEP7` reader - 7:7\\]
Enable step"]
pub type Step7R = crate::BitReader;
#[doc = "Field `STEP7` writer - 7:7\\]
Enable step"]
pub type Step7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STEP8` reader - 8:8\\]
Enable step"]
pub type Step8R = crate::BitReader;
#[doc = "Field `STEP8` writer - 8:8\\]
Enable step"]
pub type Step8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STEP9` reader - 9:9\\]
Enable step"]
pub type Step9R = crate::BitReader;
#[doc = "Field `STEP9` writer - 9:9\\]
Enable step"]
pub type Step9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STEP10` reader - 10:10\\]
Enable step"]
pub type Step10R = crate::BitReader;
#[doc = "Field `STEP10` writer - 10:10\\]
Enable step"]
pub type Step10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STEP11` reader - 11:11\\]
Enable step"]
pub type Step11R = crate::BitReader;
#[doc = "Field `STEP11` writer - 11:11\\]
Enable step"]
pub type Step11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STEP12` reader - 12:12\\]
Enable step"]
pub type Step12R = crate::BitReader;
#[doc = "Field `STEP12` writer - 12:12\\]
Enable step"]
pub type Step12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STEP13` reader - 13:13\\]
Enable step"]
pub type Step13R = crate::BitReader;
#[doc = "Field `STEP13` writer - 13:13\\]
Enable step"]
pub type Step13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STEP14` reader - 14:14\\]
Enable step"]
pub type Step14R = crate::BitReader;
#[doc = "Field `STEP14` writer - 14:14\\]
Enable step"]
pub type Step14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STEP15` reader - 15:15\\]
Enable step"]
pub type Step15R = crate::BitReader;
#[doc = "Field `STEP15` writer - 15:15\\]
Enable step"]
pub type Step15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STEP16` reader - 16:16\\]
Enable step"]
pub type Step16R = crate::BitReader;
#[doc = "Field `STEP16` writer - 16:16\\]
Enable step"]
pub type Step16W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - 1:1\\]
Enable step"]
    #[inline(always)]
    pub fn step1(&self) -> Step1R {
        Step1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Enable step"]
    #[inline(always)]
    pub fn step2(&self) -> Step2R {
        Step2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Enable step"]
    #[inline(always)]
    pub fn step3(&self) -> Step3R {
        Step3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Enable step"]
    #[inline(always)]
    pub fn step4(&self) -> Step4R {
        Step4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Enable step"]
    #[inline(always)]
    pub fn step5(&self) -> Step5R {
        Step5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Enable step"]
    #[inline(always)]
    pub fn step6(&self) -> Step6R {
        Step6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Enable step"]
    #[inline(always)]
    pub fn step7(&self) -> Step7R {
        Step7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable step"]
    #[inline(always)]
    pub fn step8(&self) -> Step8R {
        Step8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Enable step"]
    #[inline(always)]
    pub fn step9(&self) -> Step9R {
        Step9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Enable step"]
    #[inline(always)]
    pub fn step10(&self) -> Step10R {
        Step10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Enable step"]
    #[inline(always)]
    pub fn step11(&self) -> Step11R {
        Step11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Enable step"]
    #[inline(always)]
    pub fn step12(&self) -> Step12R {
        Step12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Enable step"]
    #[inline(always)]
    pub fn step13(&self) -> Step13R {
        Step13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Enable step"]
    #[inline(always)]
    pub fn step14(&self) -> Step14R {
        Step14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Enable step"]
    #[inline(always)]
    pub fn step15(&self) -> Step15R {
        Step15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Enable step"]
    #[inline(always)]
    pub fn step16(&self) -> Step16R {
        Step16R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - 1:1\\]
Enable step"]
    #[inline(always)]
    #[must_use]
    pub fn step1(&mut self) -> Step1W<AdcregsStepenableSpec> {
        Step1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Enable step"]
    #[inline(always)]
    #[must_use]
    pub fn step2(&mut self) -> Step2W<AdcregsStepenableSpec> {
        Step2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Enable step"]
    #[inline(always)]
    #[must_use]
    pub fn step3(&mut self) -> Step3W<AdcregsStepenableSpec> {
        Step3W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Enable step"]
    #[inline(always)]
    #[must_use]
    pub fn step4(&mut self) -> Step4W<AdcregsStepenableSpec> {
        Step4W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Enable step"]
    #[inline(always)]
    #[must_use]
    pub fn step5(&mut self) -> Step5W<AdcregsStepenableSpec> {
        Step5W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Enable step"]
    #[inline(always)]
    #[must_use]
    pub fn step6(&mut self) -> Step6W<AdcregsStepenableSpec> {
        Step6W::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Enable step"]
    #[inline(always)]
    #[must_use]
    pub fn step7(&mut self) -> Step7W<AdcregsStepenableSpec> {
        Step7W::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable step"]
    #[inline(always)]
    #[must_use]
    pub fn step8(&mut self) -> Step8W<AdcregsStepenableSpec> {
        Step8W::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Enable step"]
    #[inline(always)]
    #[must_use]
    pub fn step9(&mut self) -> Step9W<AdcregsStepenableSpec> {
        Step9W::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Enable step"]
    #[inline(always)]
    #[must_use]
    pub fn step10(&mut self) -> Step10W<AdcregsStepenableSpec> {
        Step10W::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Enable step"]
    #[inline(always)]
    #[must_use]
    pub fn step11(&mut self) -> Step11W<AdcregsStepenableSpec> {
        Step11W::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Enable step"]
    #[inline(always)]
    #[must_use]
    pub fn step12(&mut self) -> Step12W<AdcregsStepenableSpec> {
        Step12W::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Enable step"]
    #[inline(always)]
    #[must_use]
    pub fn step13(&mut self) -> Step13W<AdcregsStepenableSpec> {
        Step13W::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Enable step"]
    #[inline(always)]
    #[must_use]
    pub fn step14(&mut self) -> Step14W<AdcregsStepenableSpec> {
        Step14W::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Enable step"]
    #[inline(always)]
    #[must_use]
    pub fn step15(&mut self) -> Step15W<AdcregsStepenableSpec> {
        Step15W::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
Enable step"]
    #[inline(always)]
    #[must_use]
    pub fn step16(&mut self) -> Step16W<AdcregsStepenableSpec> {
        Step16W::new(self, 16)
    }
}
#[doc = "Contains the enable bit for each step in the sequencer. When all steps are disabled, the FSM will stay in IDLE state. These bits can be enabled or disabled dynamically during operation. When a write to this register occurs during operational mode, the HW will make sure the new settings are updated after the END_OF_SEQUENCE event\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_stepenable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_stepenable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcregsStepenableSpec;
impl crate::RegisterSpec for AdcregsStepenableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcregs_stepenable::R`](R) reader structure"]
impl crate::Readable for AdcregsStepenableSpec {}
#[doc = "`write(|w| ..)` method takes [`adcregs_stepenable::W`](W) writer structure"]
impl crate::Writable for AdcregsStepenableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADCREGS_STEPENABLE to value 0"]
impl crate::Resettable for AdcregsStepenableSpec {
    const RESET_VALUE: u32 = 0;
}
