#[doc = "Register `FSAS__FSAS_MMR_CFG__FSAS_GENREGS_ECC_TYPE` reader"]
pub type R = crate::R<Fsas_FsasMmrCfg_FsasGenregsEccTypeSpec>;
#[doc = "Register `FSAS__FSAS_MMR_CFG__FSAS_GENREGS_ECC_TYPE` writer"]
pub type W = crate::W<Fsas_FsasMmrCfg_FsasGenregsEccTypeSpec>;
#[doc = "Field `ECC_ERR_SEC` reader - 0:0\\]
hen set indicates that there was a single error detected for the block"]
pub type EccErrSecR = crate::BitReader;
#[doc = "Field `ECC_ERR_SEC` writer - 0:0\\]
hen set indicates that there was a single error detected for the block"]
pub type EccErrSecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_ERR_DED` reader - 1:1\\]
When set indicates that there was a double error detected for the block"]
pub type EccErrDedR = crate::BitReader;
#[doc = "Field `ECC_ERR_DED` writer - 1:1\\]
When set indicates that there was a double error detected for the block"]
pub type EccErrDedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_ERR_DA0` reader - 2:2\\]
When set indicates that there was a single error detected within the Low Data word"]
pub type EccErrDa0R = crate::BitReader;
#[doc = "Field `ECC_ERR_DA0` writer - 2:2\\]
When set indicates that there was a single error detected within the Low Data word"]
pub type EccErrDa0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_ERR_DA1` reader - 3:3\\]
When set indicates that there was a single error detected within the High Data word"]
pub type EccErrDa1R = crate::BitReader;
#[doc = "Field `ECC_ERR_DA1` writer - 3:3\\]
When set indicates that there was a single error detected within the High Data word"]
pub type EccErrDa1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_ERR_MAC` reader - 4:4\\]
When set indicates that there was a single error detected within the MAC field"]
pub type EccErrMacR = crate::BitReader;
#[doc = "Field `ECC_ERR_MAC` writer - 4:4\\]
When set indicates that there was a single error detected within the MAC field"]
pub type EccErrMacW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_ERR_ADR` reader - 5:5\\]
When set indicates that there was a single error detected within the address field"]
pub type EccErrAdrR = crate::BitReader;
#[doc = "Field `ECC_ERR_ADR` writer - 5:5\\]
When set indicates that there was a single error detected within the address field"]
pub type EccErrAdrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_ERR_VALID` reader - 31:31\\]
When set indicates that there is valid ECC error information available, Writing a one to this register will pop the top of the stack"]
pub type EccErrValidR = crate::BitReader;
#[doc = "Field `ECC_ERR_VALID` writer - 31:31\\]
When set indicates that there is valid ECC error information available, Writing a one to this register will pop the top of the stack"]
pub type EccErrValidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
hen set indicates that there was a single error detected for the block"]
    #[inline(always)]
    pub fn ecc_err_sec(&self) -> EccErrSecR {
        EccErrSecR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
When set indicates that there was a double error detected for the block"]
    #[inline(always)]
    pub fn ecc_err_ded(&self) -> EccErrDedR {
        EccErrDedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
When set indicates that there was a single error detected within the Low Data word"]
    #[inline(always)]
    pub fn ecc_err_da0(&self) -> EccErrDa0R {
        EccErrDa0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
When set indicates that there was a single error detected within the High Data word"]
    #[inline(always)]
    pub fn ecc_err_da1(&self) -> EccErrDa1R {
        EccErrDa1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
When set indicates that there was a single error detected within the MAC field"]
    #[inline(always)]
    pub fn ecc_err_mac(&self) -> EccErrMacR {
        EccErrMacR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
When set indicates that there was a single error detected within the address field"]
    #[inline(always)]
    pub fn ecc_err_adr(&self) -> EccErrAdrR {
        EccErrAdrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
When set indicates that there is valid ECC error information available, Writing a one to this register will pop the top of the stack"]
    #[inline(always)]
    pub fn ecc_err_valid(&self) -> EccErrValidR {
        EccErrValidR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
hen set indicates that there was a single error detected for the block"]
    #[inline(always)]
    #[must_use]
    pub fn ecc_err_sec(&mut self) -> EccErrSecW<Fsas_FsasMmrCfg_FsasGenregsEccTypeSpec> {
        EccErrSecW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
When set indicates that there was a double error detected for the block"]
    #[inline(always)]
    #[must_use]
    pub fn ecc_err_ded(&mut self) -> EccErrDedW<Fsas_FsasMmrCfg_FsasGenregsEccTypeSpec> {
        EccErrDedW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
When set indicates that there was a single error detected within the Low Data word"]
    #[inline(always)]
    #[must_use]
    pub fn ecc_err_da0(&mut self) -> EccErrDa0W<Fsas_FsasMmrCfg_FsasGenregsEccTypeSpec> {
        EccErrDa0W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
When set indicates that there was a single error detected within the High Data word"]
    #[inline(always)]
    #[must_use]
    pub fn ecc_err_da1(&mut self) -> EccErrDa1W<Fsas_FsasMmrCfg_FsasGenregsEccTypeSpec> {
        EccErrDa1W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
When set indicates that there was a single error detected within the MAC field"]
    #[inline(always)]
    #[must_use]
    pub fn ecc_err_mac(&mut self) -> EccErrMacW<Fsas_FsasMmrCfg_FsasGenregsEccTypeSpec> {
        EccErrMacW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
When set indicates that there was a single error detected within the address field"]
    #[inline(always)]
    #[must_use]
    pub fn ecc_err_adr(&mut self) -> EccErrAdrW<Fsas_FsasMmrCfg_FsasGenregsEccTypeSpec> {
        EccErrAdrW::new(self, 5)
    }
    #[doc = "Bit 31 - 31:31\\]
When set indicates that there is valid ECC error information available, Writing a one to this register will pop the top of the stack"]
    #[inline(always)]
    #[must_use]
    pub fn ecc_err_valid(&mut self) -> EccErrValidW<Fsas_FsasMmrCfg_FsasGenregsEccTypeSpec> {
        EccErrValidW::new(self, 31)
    }
}
#[doc = "The ERR_ECC_TYPE register holds the current top of stack ECC error info, this is only valid when the ecc_err_valid is set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_mmr_cfg__fsas_genregs_ecc_type::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_mmr_cfg__fsas_genregs_ecc_type::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fsas_FsasMmrCfg_FsasGenregsEccTypeSpec;
impl crate::RegisterSpec for Fsas_FsasMmrCfg_FsasGenregsEccTypeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsas__fsas_mmr_cfg__fsas_genregs_ecc_type::R`](R) reader structure"]
impl crate::Readable for Fsas_FsasMmrCfg_FsasGenregsEccTypeSpec {}
#[doc = "`write(|w| ..)` method takes [`fsas__fsas_mmr_cfg__fsas_genregs_ecc_type::W`](W) writer structure"]
impl crate::Writable for Fsas_FsasMmrCfg_FsasGenregsEccTypeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSAS__FSAS_MMR_CFG__FSAS_GENREGS_ECC_TYPE to value 0"]
impl crate::Resettable for Fsas_FsasMmrCfg_FsasGenregsEccTypeSpec {
    const RESET_VALUE: u32 = 0;
}
