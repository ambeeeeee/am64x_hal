#[doc = "Register `CFG0_FUSE_CRC_STAT` reader"]
pub type R = crate::R<Cfg0FuseCrcStatSpec>;
#[doc = "Register `CFG0_FUSE_CRC_STAT` writer"]
pub type W = crate::W<Cfg0FuseCrcStatSpec>;
#[doc = "Field `FUSE_CRC_STAT_CRC_ERR_1` reader - 1:1\\]
Indicates eFuse CRC error on chain 1"]
pub type FuseCrcStatCrcErr1R = crate::BitReader;
#[doc = "Field `FUSE_CRC_STAT_CRC_ERR_1` writer - 1:1\\]
Indicates eFuse CRC error on chain 1"]
pub type FuseCrcStatCrcErr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FUSE_CRC_STAT_CRC_ERR_2` reader - 2:2\\]
Indicates eFuse CRC error on chain 2"]
pub type FuseCrcStatCrcErr2R = crate::BitReader;
#[doc = "Field `FUSE_CRC_STAT_CRC_ERR_2` writer - 2:2\\]
Indicates eFuse CRC error on chain 2"]
pub type FuseCrcStatCrcErr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FUSE_CRC_STAT_CRC_ERR_3` reader - 3:3\\]
Indicates eFuse CRC error on chain 3"]
pub type FuseCrcStatCrcErr3R = crate::BitReader;
#[doc = "Field `FUSE_CRC_STAT_CRC_ERR_3` writer - 3:3\\]
Indicates eFuse CRC error on chain 3"]
pub type FuseCrcStatCrcErr3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FUSE_CRC_STAT_CRC_ERR_4` reader - 4:4\\]
Indicates eFuse CRC error on chain 4"]
pub type FuseCrcStatCrcErr4R = crate::BitReader;
#[doc = "Field `FUSE_CRC_STAT_CRC_ERR_4` writer - 4:4\\]
Indicates eFuse CRC error on chain 4"]
pub type FuseCrcStatCrcErr4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FUSE_CRC_STAT_CRC_ERR_5` reader - 5:5\\]
Indicates eFuse CRC error on chain 5"]
pub type FuseCrcStatCrcErr5R = crate::BitReader;
#[doc = "Field `FUSE_CRC_STAT_CRC_ERR_5` writer - 5:5\\]
Indicates eFuse CRC error on chain 5"]
pub type FuseCrcStatCrcErr5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FUSE_CRC_STAT_CRC_ERR_6` reader - 6:6\\]
Indicates eFuse CRC error on chain 6"]
pub type FuseCrcStatCrcErr6R = crate::BitReader;
#[doc = "Field `FUSE_CRC_STAT_CRC_ERR_6` writer - 6:6\\]
Indicates eFuse CRC error on chain 6"]
pub type FuseCrcStatCrcErr6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FUSE_CRC_STAT_CRC_ERR_7` reader - 7:7\\]
Indicates eFuse CRC error on chain 7"]
pub type FuseCrcStatCrcErr7R = crate::BitReader;
#[doc = "Field `FUSE_CRC_STAT_CRC_ERR_7` writer - 7:7\\]
Indicates eFuse CRC error on chain 7"]
pub type FuseCrcStatCrcErr7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - 1:1\\]
Indicates eFuse CRC error on chain 1"]
    #[inline(always)]
    pub fn fuse_crc_stat_crc_err_1(&self) -> FuseCrcStatCrcErr1R {
        FuseCrcStatCrcErr1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Indicates eFuse CRC error on chain 2"]
    #[inline(always)]
    pub fn fuse_crc_stat_crc_err_2(&self) -> FuseCrcStatCrcErr2R {
        FuseCrcStatCrcErr2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Indicates eFuse CRC error on chain 3"]
    #[inline(always)]
    pub fn fuse_crc_stat_crc_err_3(&self) -> FuseCrcStatCrcErr3R {
        FuseCrcStatCrcErr3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Indicates eFuse CRC error on chain 4"]
    #[inline(always)]
    pub fn fuse_crc_stat_crc_err_4(&self) -> FuseCrcStatCrcErr4R {
        FuseCrcStatCrcErr4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Indicates eFuse CRC error on chain 5"]
    #[inline(always)]
    pub fn fuse_crc_stat_crc_err_5(&self) -> FuseCrcStatCrcErr5R {
        FuseCrcStatCrcErr5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Indicates eFuse CRC error on chain 6"]
    #[inline(always)]
    pub fn fuse_crc_stat_crc_err_6(&self) -> FuseCrcStatCrcErr6R {
        FuseCrcStatCrcErr6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Indicates eFuse CRC error on chain 7"]
    #[inline(always)]
    pub fn fuse_crc_stat_crc_err_7(&self) -> FuseCrcStatCrcErr7R {
        FuseCrcStatCrcErr7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - 1:1\\]
Indicates eFuse CRC error on chain 1"]
    #[inline(always)]
    #[must_use]
    pub fn fuse_crc_stat_crc_err_1(&mut self) -> FuseCrcStatCrcErr1W<Cfg0FuseCrcStatSpec> {
        FuseCrcStatCrcErr1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Indicates eFuse CRC error on chain 2"]
    #[inline(always)]
    #[must_use]
    pub fn fuse_crc_stat_crc_err_2(&mut self) -> FuseCrcStatCrcErr2W<Cfg0FuseCrcStatSpec> {
        FuseCrcStatCrcErr2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Indicates eFuse CRC error on chain 3"]
    #[inline(always)]
    #[must_use]
    pub fn fuse_crc_stat_crc_err_3(&mut self) -> FuseCrcStatCrcErr3W<Cfg0FuseCrcStatSpec> {
        FuseCrcStatCrcErr3W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Indicates eFuse CRC error on chain 4"]
    #[inline(always)]
    #[must_use]
    pub fn fuse_crc_stat_crc_err_4(&mut self) -> FuseCrcStatCrcErr4W<Cfg0FuseCrcStatSpec> {
        FuseCrcStatCrcErr4W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Indicates eFuse CRC error on chain 5"]
    #[inline(always)]
    #[must_use]
    pub fn fuse_crc_stat_crc_err_5(&mut self) -> FuseCrcStatCrcErr5W<Cfg0FuseCrcStatSpec> {
        FuseCrcStatCrcErr5W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Indicates eFuse CRC error on chain 6"]
    #[inline(always)]
    #[must_use]
    pub fn fuse_crc_stat_crc_err_6(&mut self) -> FuseCrcStatCrcErr6W<Cfg0FuseCrcStatSpec> {
        FuseCrcStatCrcErr6W::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Indicates eFuse CRC error on chain 7"]
    #[inline(always)]
    #[must_use]
    pub fn fuse_crc_stat_crc_err_7(&mut self) -> FuseCrcStatCrcErr7W<Cfg0FuseCrcStatSpec> {
        FuseCrcStatCrcErr7W::new(self, 7)
    }
}
#[doc = "CFG0_FUSE_CRC_STAT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_fuse_crc_stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_fuse_crc_stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0FuseCrcStatSpec;
impl crate::RegisterSpec for Cfg0FuseCrcStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_fuse_crc_stat::R`](R) reader structure"]
impl crate::Readable for Cfg0FuseCrcStatSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_fuse_crc_stat::W`](W) writer structure"]
impl crate::Writable for Cfg0FuseCrcStatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_FUSE_CRC_STAT to value 0"]
impl crate::Resettable for Cfg0FuseCrcStatSpec {
    const RESET_VALUE: u32 = 0;
}
