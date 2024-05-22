#[doc = "Register `CFG_I2C_CON` reader"]
pub type R = crate::R<CfgI2cConSpec>;
#[doc = "Register `CFG_I2C_CON` writer"]
pub type W = crate::W<CfgI2cConSpec>;
#[doc = "Field `STT` reader - 0:0\\]
Start condition \\[master mode only\\]"]
pub type SttR = crate::BitReader;
#[doc = "Field `STT` writer - 0:0\\]
Start condition \\[master mode only\\]"]
pub type SttW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STP` reader - 1:1\\]
Stop condition \\[master mode only\\]"]
pub type StpR = crate::BitReader;
#[doc = "Field `STP` writer - 1:1\\]
Stop condition \\[master mode only\\]"]
pub type StpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XOA3` reader - 4:4\\]
Expand Own address 3"]
pub type Xoa3R = crate::BitReader;
#[doc = "Field `XOA3` writer - 4:4\\]
Expand Own address 3"]
pub type Xoa3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XOA2` reader - 5:5\\]
Expand Own address 2"]
pub type Xoa2R = crate::BitReader;
#[doc = "Field `XOA2` writer - 5:5\\]
Expand Own address 2"]
pub type Xoa2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XOA1` reader - 6:6\\]
Expand Own address 1"]
pub type Xoa1R = crate::BitReader;
#[doc = "Field `XOA1` writer - 6:6\\]
Expand Own address 1"]
pub type Xoa1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XOA0` reader - 7:7\\]
Expand Own address 0"]
pub type Xoa0R = crate::BitReader;
#[doc = "Field `XOA0` writer - 7:7\\]
Expand Own address 0"]
pub type Xoa0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XSA` reader - 8:8\\]
Expand Slave address"]
pub type XsaR = crate::BitReader;
#[doc = "Field `XSA` writer - 8:8\\]
Expand Slave address"]
pub type XsaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRX` reader - 9:9\\]
Transmitter/Receiver mode \\[master mode only\\]"]
pub type TrxR = crate::BitReader;
#[doc = "Field `TRX` writer - 9:9\\]
Transmitter/Receiver mode \\[master mode only\\]"]
pub type TrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MST` reader - 10:10\\]
Master/slave mode"]
pub type MstR = crate::BitReader;
#[doc = "Field `MST` writer - 10:10\\]
Master/slave mode"]
pub type MstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STB` reader - 11:11\\]
Start byte mode \\[master mode only\\]"]
pub type StbR = crate::BitReader;
#[doc = "Field `STB` writer - 11:11\\]
Start byte mode \\[master mode only\\]"]
pub type StbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPMODE` reader - 13:12\\]
Operation mode selection"]
pub type OpmodeR = crate::FieldReader;
#[doc = "Field `OPMODE` writer - 13:12\\]
Operation mode selection"]
pub type OpmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2C_EN` reader - 15:15\\]
I2C module enable"]
pub type I2cEnR = crate::BitReader;
#[doc = "Field `I2C_EN` writer - 15:15\\]
I2C module enable"]
pub type I2cEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Start condition \\[master mode only\\]"]
    #[inline(always)]
    pub fn stt(&self) -> SttR {
        SttR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Stop condition \\[master mode only\\]"]
    #[inline(always)]
    pub fn stp(&self) -> StpR {
        StpR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Expand Own address 3"]
    #[inline(always)]
    pub fn xoa3(&self) -> Xoa3R {
        Xoa3R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Expand Own address 2"]
    #[inline(always)]
    pub fn xoa2(&self) -> Xoa2R {
        Xoa2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Expand Own address 1"]
    #[inline(always)]
    pub fn xoa1(&self) -> Xoa1R {
        Xoa1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Expand Own address 0"]
    #[inline(always)]
    pub fn xoa0(&self) -> Xoa0R {
        Xoa0R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Expand Slave address"]
    #[inline(always)]
    pub fn xsa(&self) -> XsaR {
        XsaR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Transmitter/Receiver mode \\[master mode only\\]"]
    #[inline(always)]
    pub fn trx(&self) -> TrxR {
        TrxR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Master/slave mode"]
    #[inline(always)]
    pub fn mst(&self) -> MstR {
        MstR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Start byte mode \\[master mode only\\]"]
    #[inline(always)]
    pub fn stb(&self) -> StbR {
        StbR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Operation mode selection"]
    #[inline(always)]
    pub fn opmode(&self) -> OpmodeR {
        OpmodeR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
I2C module enable"]
    #[inline(always)]
    pub fn i2c_en(&self) -> I2cEnR {
        I2cEnR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Start condition \\[master mode only\\]"]
    #[inline(always)]
    #[must_use]
    pub fn stt(&mut self) -> SttW<CfgI2cConSpec> {
        SttW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Stop condition \\[master mode only\\]"]
    #[inline(always)]
    #[must_use]
    pub fn stp(&mut self) -> StpW<CfgI2cConSpec> {
        StpW::new(self, 1)
    }
    #[doc = "Bit 4 - 4:4\\]
Expand Own address 3"]
    #[inline(always)]
    #[must_use]
    pub fn xoa3(&mut self) -> Xoa3W<CfgI2cConSpec> {
        Xoa3W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Expand Own address 2"]
    #[inline(always)]
    #[must_use]
    pub fn xoa2(&mut self) -> Xoa2W<CfgI2cConSpec> {
        Xoa2W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Expand Own address 1"]
    #[inline(always)]
    #[must_use]
    pub fn xoa1(&mut self) -> Xoa1W<CfgI2cConSpec> {
        Xoa1W::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Expand Own address 0"]
    #[inline(always)]
    #[must_use]
    pub fn xoa0(&mut self) -> Xoa0W<CfgI2cConSpec> {
        Xoa0W::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Expand Slave address"]
    #[inline(always)]
    #[must_use]
    pub fn xsa(&mut self) -> XsaW<CfgI2cConSpec> {
        XsaW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Transmitter/Receiver mode \\[master mode only\\]"]
    #[inline(always)]
    #[must_use]
    pub fn trx(&mut self) -> TrxW<CfgI2cConSpec> {
        TrxW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Master/slave mode"]
    #[inline(always)]
    #[must_use]
    pub fn mst(&mut self) -> MstW<CfgI2cConSpec> {
        MstW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Start byte mode \\[master mode only\\]"]
    #[inline(always)]
    #[must_use]
    pub fn stb(&mut self) -> StbW<CfgI2cConSpec> {
        StbW::new(self, 11)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Operation mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn opmode(&mut self) -> OpmodeW<CfgI2cConSpec> {
        OpmodeW::new(self, 12)
    }
    #[doc = "Bit 15 - 15:15\\]
I2C module enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_en(&mut self) -> I2cEnW<CfgI2cConSpec> {
        I2cEnW::new(self, 15)
    }
}
#[doc = "I2C configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_con::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_con::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgI2cConSpec;
impl crate::RegisterSpec for CfgI2cConSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_i2c_con::R`](R) reader structure"]
impl crate::Readable for CfgI2cConSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_i2c_con::W`](W) writer structure"]
impl crate::Writable for CfgI2cConSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_I2C_CON to value 0"]
impl crate::Resettable for CfgI2cConSpec {
    const RESET_VALUE: u32 = 0;
}
