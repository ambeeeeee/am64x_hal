#[doc = "Register `FSAS__FSAS_MMR_CFG__FSAS_GENREGS_SYSCONFIG` reader"]
pub type R = crate::R<Fsas_FsasMmrCfg_FsasGenregsSysconfigSpec>;
#[doc = "Register `FSAS__FSAS_MMR_CFG__FSAS_GENREGS_SYSCONFIG` writer"]
pub type W = crate::W<Fsas_FsasMmrCfg_FsasGenregsSysconfigSpec>;
#[doc = "Field `ECC_EN` reader - 0:0\\]
0 ECC disabled. 1 ECC enabled"]
pub type EccEnR = crate::BitReader;
#[doc = "Field `ECC_EN` writer - 0:0\\]
0 ECC disabled. 1 ECC enabled"]
pub type EccEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSS_AES_EN_IPCFG` reader - 2:2\\]
1 select security, 0 disable security"]
pub type FssAesEnIpcfgR = crate::BitReader;
#[doc = "Field `FSS_AES_EN_IPCFG` writer - 2:2\\]
1 select security, 0 disable security"]
pub type FssAesEnIpcfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_DISABLE_ADR` reader - 3:3\\]
0 Block address within ECC calculation, 1 Block address not within ECC calculation"]
pub type EccDisableAdrR = crate::BitReader;
#[doc = "Field `ECC_DISABLE_ADR` writer - 3:3\\]
0 Block address within ECC calculation, 1 Block address not within ECC calculation"]
pub type EccDisableAdrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSPI_DDR_DISABLE_MODE` reader - 6:6\\]
0 OSPI DDR mode enabled. 1 OSPI DDR mode disabled"]
pub type OspiDdrDisableModeR = crate::BitReader;
#[doc = "Field `OSPI_DDR_DISABLE_MODE` writer - 6:6\\]
0 OSPI DDR mode enabled. 1 OSPI DDR mode disabled"]
pub type OspiDdrDisableModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISXIP` reader - 7:7\\]
0 XIP Prefetch Enabled. 1 XIP prefetch disabled"]
pub type DisxipR = crate::BitReader;
#[doc = "Field `DISXIP` writer - 7:7\\]
0 XIP Prefetch Enabled. 1 XIP prefetch disabled"]
pub type DisxipW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSPI_32B_DISABLE_MODE` reader - 8:8\\]
0 OSPI 32bit mode enabled. 1 OSPI 32bit mode disabled"]
pub type Ospi32bDisableModeR = crate::BitReader;
#[doc = "Field `OSPI_32B_DISABLE_MODE` writer - 8:8\\]
0 OSPI 32bit mode enabled. 1 OSPI 32bit mode disabled"]
pub type Ospi32bDisableModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
0 ECC disabled. 1 ECC enabled"]
    #[inline(always)]
    pub fn ecc_en(&self) -> EccEnR {
        EccEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
1 select security, 0 disable security"]
    #[inline(always)]
    pub fn fss_aes_en_ipcfg(&self) -> FssAesEnIpcfgR {
        FssAesEnIpcfgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
0 Block address within ECC calculation, 1 Block address not within ECC calculation"]
    #[inline(always)]
    pub fn ecc_disable_adr(&self) -> EccDisableAdrR {
        EccDisableAdrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
0 OSPI DDR mode enabled. 1 OSPI DDR mode disabled"]
    #[inline(always)]
    pub fn ospi_ddr_disable_mode(&self) -> OspiDdrDisableModeR {
        OspiDdrDisableModeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
0 XIP Prefetch Enabled. 1 XIP prefetch disabled"]
    #[inline(always)]
    pub fn disxip(&self) -> DisxipR {
        DisxipR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
0 OSPI 32bit mode enabled. 1 OSPI 32bit mode disabled"]
    #[inline(always)]
    pub fn ospi_32b_disable_mode(&self) -> Ospi32bDisableModeR {
        Ospi32bDisableModeR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
0 ECC disabled. 1 ECC enabled"]
    #[inline(always)]
    #[must_use]
    pub fn ecc_en(&mut self) -> EccEnW<Fsas_FsasMmrCfg_FsasGenregsSysconfigSpec> {
        EccEnW::new(self, 0)
    }
    #[doc = "Bit 2 - 2:2\\]
1 select security, 0 disable security"]
    #[inline(always)]
    #[must_use]
    pub fn fss_aes_en_ipcfg(&mut self) -> FssAesEnIpcfgW<Fsas_FsasMmrCfg_FsasGenregsSysconfigSpec> {
        FssAesEnIpcfgW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
0 Block address within ECC calculation, 1 Block address not within ECC calculation"]
    #[inline(always)]
    #[must_use]
    pub fn ecc_disable_adr(&mut self) -> EccDisableAdrW<Fsas_FsasMmrCfg_FsasGenregsSysconfigSpec> {
        EccDisableAdrW::new(self, 3)
    }
    #[doc = "Bit 6 - 6:6\\]
0 OSPI DDR mode enabled. 1 OSPI DDR mode disabled"]
    #[inline(always)]
    #[must_use]
    pub fn ospi_ddr_disable_mode(
        &mut self,
    ) -> OspiDdrDisableModeW<Fsas_FsasMmrCfg_FsasGenregsSysconfigSpec> {
        OspiDdrDisableModeW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
0 XIP Prefetch Enabled. 1 XIP prefetch disabled"]
    #[inline(always)]
    #[must_use]
    pub fn disxip(&mut self) -> DisxipW<Fsas_FsasMmrCfg_FsasGenregsSysconfigSpec> {
        DisxipW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
0 OSPI 32bit mode enabled. 1 OSPI 32bit mode disabled"]
    #[inline(always)]
    #[must_use]
    pub fn ospi_32b_disable_mode(
        &mut self,
    ) -> Ospi32bDisableModeW<Fsas_FsasMmrCfg_FsasGenregsSysconfigSpec> {
        Ospi32bDisableModeW::new(self, 8)
    }
}
#[doc = "Controls various parameters of the cotroller state.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_mmr_cfg__fsas_genregs_sysconfig::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_mmr_cfg__fsas_genregs_sysconfig::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fsas_FsasMmrCfg_FsasGenregsSysconfigSpec;
impl crate::RegisterSpec for Fsas_FsasMmrCfg_FsasGenregsSysconfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsas__fsas_mmr_cfg__fsas_genregs_sysconfig::R`](R) reader structure"]
impl crate::Readable for Fsas_FsasMmrCfg_FsasGenregsSysconfigSpec {}
#[doc = "`write(|w| ..)` method takes [`fsas__fsas_mmr_cfg__fsas_genregs_sysconfig::W`](W) writer structure"]
impl crate::Writable for Fsas_FsasMmrCfg_FsasGenregsSysconfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSAS__FSAS_MMR_CFG__FSAS_GENREGS_SYSCONFIG to value 0"]
impl crate::Resettable for Fsas_FsasMmrCfg_FsasGenregsSysconfigSpec {
    const RESET_VALUE: u32 = 0;
}
