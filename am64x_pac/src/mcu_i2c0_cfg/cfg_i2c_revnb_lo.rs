#[doc = "Register `CFG_I2C_REVNB_LO` reader"]
pub type R = crate::R<CfgI2cRevnbLoSpec>;
#[doc = "Register `CFG_I2C_REVNB_LO` writer"]
pub type W = crate::W<CfgI2cRevnbLoSpec>;
#[doc = "Field `MINOR` reader - 5:0\\]
Minor Revision This field changes when features are scaled up or down This field does not change due to bug fix, or major feature change"]
pub type MinorR = crate::FieldReader;
#[doc = "Field `MINOR` writer - 5:0\\]
Minor Revision This field changes when features are scaled up or down This field does not change due to bug fix, or major feature change"]
pub type MinorW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CUSTOM` reader - 7:6\\]
Indicates a special version for a particular device Consequence of use may avoid use of standard Chip Support Library \\[CSL\\]
/ Drivers 0 if non-custom"]
pub type CustomR = crate::FieldReader;
#[doc = "Field `CUSTOM` writer - 7:6\\]
Indicates a special version for a particular device Consequence of use may avoid use of standard Chip Support Library \\[CSL\\]
/ Drivers 0 if non-custom"]
pub type CustomW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MAJOR` reader - 10:8\\]
Major Revision This field changes when there is a major feature change This field does not change due to bug fix, or minor feature change"]
pub type MajorR = crate::FieldReader;
#[doc = "Field `MAJOR` writer - 10:8\\]
Major Revision This field changes when there is a major feature change This field does not change due to bug fix, or minor feature change"]
pub type MajorW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RTL` reader - 15:11\\]
RTL version This field changes on bug fix, and resets to"]
pub type RtlR = crate::FieldReader;
#[doc = "Field `RTL` writer - 15:11\\]
RTL version This field changes on bug fix, and resets to"]
pub type RtlW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Minor Revision This field changes when features are scaled up or down This field does not change due to bug fix, or major feature change"]
    #[inline(always)]
    pub fn minor(&self) -> MinorR {
        MinorR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Indicates a special version for a particular device Consequence of use may avoid use of standard Chip Support Library \\[CSL\\]
/ Drivers 0 if non-custom"]
    #[inline(always)]
    pub fn custom(&self) -> CustomR {
        CustomR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Major Revision This field changes when there is a major feature change This field does not change due to bug fix, or minor feature change"]
    #[inline(always)]
    pub fn major(&self) -> MajorR {
        MajorR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
RTL version This field changes on bug fix, and resets to"]
    #[inline(always)]
    pub fn rtl(&self) -> RtlR {
        RtlR::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Minor Revision This field changes when features are scaled up or down This field does not change due to bug fix, or major feature change"]
    #[inline(always)]
    #[must_use]
    pub fn minor(&mut self) -> MinorW<CfgI2cRevnbLoSpec> {
        MinorW::new(self, 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Indicates a special version for a particular device Consequence of use may avoid use of standard Chip Support Library \\[CSL\\]
/ Drivers 0 if non-custom"]
    #[inline(always)]
    #[must_use]
    pub fn custom(&mut self) -> CustomW<CfgI2cRevnbLoSpec> {
        CustomW::new(self, 6)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Major Revision This field changes when there is a major feature change This field does not change due to bug fix, or minor feature change"]
    #[inline(always)]
    #[must_use]
    pub fn major(&mut self) -> MajorW<CfgI2cRevnbLoSpec> {
        MajorW::new(self, 8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
RTL version This field changes on bug fix, and resets to"]
    #[inline(always)]
    #[must_use]
    pub fn rtl(&mut self) -> RtlW<CfgI2cRevnbLoSpec> {
        RtlW::new(self, 11)
    }
}
#[doc = "Revision Number register (Low)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_revnb_lo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_revnb_lo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgI2cRevnbLoSpec;
impl crate::RegisterSpec for CfgI2cRevnbLoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_i2c_revnb_lo::R`](R) reader structure"]
impl crate::Readable for CfgI2cRevnbLoSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_i2c_revnb_lo::W`](W) writer structure"]
impl crate::Writable for CfgI2cRevnbLoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_I2C_REVNB_LO to value 0x0812"]
impl crate::Resettable for CfgI2cRevnbLoSpec {
    const RESET_VALUE: u32 = 0x0812;
}
