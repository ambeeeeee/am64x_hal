#[doc = "Register `BLAZAR_IRAM_RAM_REG` reader"]
pub type R = crate::R<BlazarIramRamRegSpec>;
#[doc = "Register `BLAZAR_IRAM_RAM_REG` writer"]
pub type W = crate::W<BlazarIramRamRegSpec>;
#[doc = "Field `BYTE0` reader - 7:0\\]
This is the LS byte"]
pub type Byte0R = crate::FieldReader;
#[doc = "Field `BYTE0` writer - 7:0\\]
This is the LS byte"]
pub type Byte0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BYTE1` reader - 15:8\\]
This is the LM byte"]
pub type Byte1R = crate::FieldReader;
#[doc = "Field `BYTE1` writer - 15:8\\]
This is the LM byte"]
pub type Byte1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BYTE2` reader - 23:16\\]
This is the UM byte"]
pub type Byte2R = crate::FieldReader;
#[doc = "Field `BYTE2` writer - 23:16\\]
This is the UM byte"]
pub type Byte2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BYTE3` reader - 31:24\\]
This is the MS byte"]
pub type Byte3R = crate::FieldReader;
#[doc = "Field `BYTE3` writer - 31:24\\]
This is the MS byte"]
pub type Byte3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
This is the LS byte"]
    #[inline(always)]
    pub fn byte0(&self) -> Byte0R {
        Byte0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
This is the LM byte"]
    #[inline(always)]
    pub fn byte1(&self) -> Byte1R {
        Byte1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
This is the UM byte"]
    #[inline(always)]
    pub fn byte2(&self) -> Byte2R {
        Byte2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
This is the MS byte"]
    #[inline(always)]
    pub fn byte3(&self) -> Byte3R {
        Byte3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
This is the LS byte"]
    #[inline(always)]
    #[must_use]
    pub fn byte0(&mut self) -> Byte0W<BlazarIramRamRegSpec> {
        Byte0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
This is the LM byte"]
    #[inline(always)]
    #[must_use]
    pub fn byte1(&mut self) -> Byte1W<BlazarIramRamRegSpec> {
        Byte1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
This is the UM byte"]
    #[inline(always)]
    #[must_use]
    pub fn byte2(&mut self) -> Byte2W<BlazarIramRamRegSpec> {
        Byte2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
This is the MS byte"]
    #[inline(always)]
    #[must_use]
    pub fn byte3(&mut self) -> Byte3W<BlazarIramRamRegSpec> {
        Byte3W::new(self, 24)
    }
}
#[doc = "The RAM memory words provide memory mapped random access data storage.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blazar_iram_ram_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blazar_iram_ram_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlazarIramRamRegSpec;
impl crate::RegisterSpec for BlazarIramRamRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blazar_iram_ram_reg::R`](R) reader structure"]
impl crate::Readable for BlazarIramRamRegSpec {}
#[doc = "`write(|w| ..)` method takes [`blazar_iram_ram_reg::W`](W) writer structure"]
impl crate::Writable for BlazarIramRamRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BLAZAR_IRAM_RAM_REG to value 0"]
impl crate::Resettable for BlazarIramRamRegSpec {
    const RESET_VALUE: u32 = 0;
}
