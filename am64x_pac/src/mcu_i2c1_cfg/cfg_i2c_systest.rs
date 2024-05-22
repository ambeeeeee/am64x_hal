#[doc = "Register `CFG_I2C_SYSTEST` reader"]
pub type R = crate::R<CfgI2cSystestSpec>;
#[doc = "Register `CFG_I2C_SYSTEST` writer"]
pub type W = crate::W<CfgI2cSystestSpec>;
#[doc = "Field `SDA_O` reader - 0:0\\]
SDA line drive output value"]
pub type SdaOR = crate::BitReader;
#[doc = "Field `SDA_O` writer - 0:0\\]
SDA line drive output value"]
pub type SdaOW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDA_I` reader - 1:1\\]
SDA line sense input value"]
pub type SdaIR = crate::BitReader;
#[doc = "Field `SDA_I` writer - 1:1\\]
SDA line sense input value"]
pub type SdaIW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCL_O` reader - 2:2\\]
SCL line drive output value"]
pub type SclOR = crate::BitReader;
#[doc = "Field `SCL_O` writer - 2:2\\]
SCL line drive output value"]
pub type SclOW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCL_I` reader - 3:3\\]
SCL line sense input value"]
pub type SclIR = crate::BitReader;
#[doc = "Field `SCL_I` writer - 3:3\\]
SCL line sense input value"]
pub type SclIW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCCB_E_O` reader - 4:4\\]
SCCB_E line sense output value"]
pub type SccbEOR = crate::BitReader;
#[doc = "Field `SCCB_E_O` writer - 4:4\\]
SCCB_E line sense output value"]
pub type SccbEOW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDA_O_FUNC` reader - 5:5\\]
SDA line output value \\[functional mode\\]"]
pub type SdaOFuncR = crate::BitReader;
#[doc = "Field `SDA_O_FUNC` writer - 5:5\\]
SDA line output value \\[functional mode\\]"]
pub type SdaOFuncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDA_I_FUNC` reader - 6:6\\]
SDA line input value \\[functional mode\\]"]
pub type SdaIFuncR = crate::BitReader;
#[doc = "Field `SDA_I_FUNC` writer - 6:6\\]
SDA line input value \\[functional mode\\]"]
pub type SdaIFuncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCL_O_FUNC` reader - 7:7\\]
SCL line output value \\[functional mode\\]"]
pub type SclOFuncR = crate::BitReader;
#[doc = "Field `SCL_O_FUNC` writer - 7:7\\]
SCL line output value \\[functional mode\\]"]
pub type SclOFuncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCL_I_FUNC` reader - 8:8\\]
SCL line input value \\[functional mode\\]"]
pub type SclIFuncR = crate::BitReader;
#[doc = "Field `SCL_I_FUNC` writer - 8:8\\]
SCL line input value \\[functional mode\\]"]
pub type SclIFuncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSB` reader - 11:11\\]
Set status bits"]
pub type SsbR = crate::BitReader;
#[doc = "Field `SSB` writer - 11:11\\]
Set status bits"]
pub type SsbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMODE` reader - 13:12\\]
Test mode select"]
pub type TmodeR = crate::FieldReader;
#[doc = "Field `TMODE` writer - 13:12\\]
Test mode select"]
pub type TmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FREE` reader - 14:14\\]
Free running mode \\[on breakpoint\\]"]
pub type FreeR = crate::BitReader;
#[doc = "Field `FREE` writer - 14:14\\]
Free running mode \\[on breakpoint\\]"]
pub type FreeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ST_EN` reader - 15:15\\]
System test enable"]
pub type StEnR = crate::BitReader;
#[doc = "Field `ST_EN` writer - 15:15\\]
System test enable"]
pub type StEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
SDA line drive output value"]
    #[inline(always)]
    pub fn sda_o(&self) -> SdaOR {
        SdaOR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
SDA line sense input value"]
    #[inline(always)]
    pub fn sda_i(&self) -> SdaIR {
        SdaIR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
SCL line drive output value"]
    #[inline(always)]
    pub fn scl_o(&self) -> SclOR {
        SclOR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
SCL line sense input value"]
    #[inline(always)]
    pub fn scl_i(&self) -> SclIR {
        SclIR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
SCCB_E line sense output value"]
    #[inline(always)]
    pub fn sccb_e_o(&self) -> SccbEOR {
        SccbEOR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
SDA line output value \\[functional mode\\]"]
    #[inline(always)]
    pub fn sda_o_func(&self) -> SdaOFuncR {
        SdaOFuncR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
SDA line input value \\[functional mode\\]"]
    #[inline(always)]
    pub fn sda_i_func(&self) -> SdaIFuncR {
        SdaIFuncR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
SCL line output value \\[functional mode\\]"]
    #[inline(always)]
    pub fn scl_o_func(&self) -> SclOFuncR {
        SclOFuncR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
SCL line input value \\[functional mode\\]"]
    #[inline(always)]
    pub fn scl_i_func(&self) -> SclIFuncR {
        SclIFuncR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Set status bits"]
    #[inline(always)]
    pub fn ssb(&self) -> SsbR {
        SsbR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Test mode select"]
    #[inline(always)]
    pub fn tmode(&self) -> TmodeR {
        TmodeR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
Free running mode \\[on breakpoint\\]"]
    #[inline(always)]
    pub fn free(&self) -> FreeR {
        FreeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
System test enable"]
    #[inline(always)]
    pub fn st_en(&self) -> StEnR {
        StEnR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
SDA line drive output value"]
    #[inline(always)]
    #[must_use]
    pub fn sda_o(&mut self) -> SdaOW<CfgI2cSystestSpec> {
        SdaOW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
SDA line sense input value"]
    #[inline(always)]
    #[must_use]
    pub fn sda_i(&mut self) -> SdaIW<CfgI2cSystestSpec> {
        SdaIW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
SCL line drive output value"]
    #[inline(always)]
    #[must_use]
    pub fn scl_o(&mut self) -> SclOW<CfgI2cSystestSpec> {
        SclOW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
SCL line sense input value"]
    #[inline(always)]
    #[must_use]
    pub fn scl_i(&mut self) -> SclIW<CfgI2cSystestSpec> {
        SclIW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
SCCB_E line sense output value"]
    #[inline(always)]
    #[must_use]
    pub fn sccb_e_o(&mut self) -> SccbEOW<CfgI2cSystestSpec> {
        SccbEOW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
SDA line output value \\[functional mode\\]"]
    #[inline(always)]
    #[must_use]
    pub fn sda_o_func(&mut self) -> SdaOFuncW<CfgI2cSystestSpec> {
        SdaOFuncW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
SDA line input value \\[functional mode\\]"]
    #[inline(always)]
    #[must_use]
    pub fn sda_i_func(&mut self) -> SdaIFuncW<CfgI2cSystestSpec> {
        SdaIFuncW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
SCL line output value \\[functional mode\\]"]
    #[inline(always)]
    #[must_use]
    pub fn scl_o_func(&mut self) -> SclOFuncW<CfgI2cSystestSpec> {
        SclOFuncW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
SCL line input value \\[functional mode\\]"]
    #[inline(always)]
    #[must_use]
    pub fn scl_i_func(&mut self) -> SclIFuncW<CfgI2cSystestSpec> {
        SclIFuncW::new(self, 8)
    }
    #[doc = "Bit 11 - 11:11\\]
Set status bits"]
    #[inline(always)]
    #[must_use]
    pub fn ssb(&mut self) -> SsbW<CfgI2cSystestSpec> {
        SsbW::new(self, 11)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Test mode select"]
    #[inline(always)]
    #[must_use]
    pub fn tmode(&mut self) -> TmodeW<CfgI2cSystestSpec> {
        TmodeW::new(self, 12)
    }
    #[doc = "Bit 14 - 14:14\\]
Free running mode \\[on breakpoint\\]"]
    #[inline(always)]
    #[must_use]
    pub fn free(&mut self) -> FreeW<CfgI2cSystestSpec> {
        FreeW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
System test enable"]
    #[inline(always)]
    #[must_use]
    pub fn st_en(&mut self) -> StEnW<CfgI2cSystestSpec> {
        StEnW::new(self, 15)
    }
}
#[doc = "I2C System Test Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_systest::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_systest::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgI2cSystestSpec;
impl crate::RegisterSpec for CfgI2cSystestSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_i2c_systest::R`](R) reader structure"]
impl crate::Readable for CfgI2cSystestSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_i2c_systest::W`](W) writer structure"]
impl crate::Writable for CfgI2cSystestSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_I2C_SYSTEST to value 0x01e0"]
impl crate::Resettable for CfgI2cSystestSpec {
    const RESET_VALUE: u32 = 0x01e0;
}
