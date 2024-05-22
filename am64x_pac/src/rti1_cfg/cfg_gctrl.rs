#[doc = "Register `CFG_GCTRL` reader"]
pub type R = crate::R<CfgGctrlSpec>;
#[doc = "Register `CFG_GCTRL` writer"]
pub type W = crate::W<CfgGctrlSpec>;
#[doc = "Field `CNT0EN` reader - 0:0\\]
The CNT0EN bit starts and stops the operation of counter block 0 (UC0 and FRC0). User and privilege mode (read): 0 = counters are stopped 1 = counters are running Privilege mode (write): 0 = stop counters 1 = start counters"]
pub type Cnt0enR = crate::BitReader;
#[doc = "Field `CNT0EN` writer - 0:0\\]
The CNT0EN bit starts and stops the operation of counter block 0 (UC0 and FRC0). User and privilege mode (read): 0 = counters are stopped 1 = counters are running Privilege mode (write): 0 = stop counters 1 = start counters"]
pub type Cnt0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNT1EN` reader - 1:1\\]
The CNT1EN bit starts and stops the operation of counter block 1 (UC1 and FRC1). User and privilege mode (read): 0 = counters are stopped 1 = counters are running Privilege mode (write): 0 = stop counters 1 = start counters"]
pub type Cnt1enR = crate::BitReader;
#[doc = "Field `CNT1EN` writer - 1:1\\]
The CNT1EN bit starts and stops the operation of counter block 1 (UC1 and FRC1). User and privilege mode (read): 0 = counters are stopped 1 = counters are running Privilege mode (write): 0 = stop counters 1 = start counters"]
pub type Cnt1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COS` reader - 15:15\\]
This bit determines if both counters are stopped when the device goes into debug mode or if they continue counting. User and privilege mode (read): 0 = counters are stopped while in debug mode 1 = counters are running while in debug mode Privilege mode (write): 0 = stop counters in debug mode 1 = continue counting in debug mode"]
pub type CosR = crate::BitReader;
#[doc = "Field `COS` writer - 15:15\\]
This bit determines if both counters are stopped when the device goes into debug mode or if they continue counting. User and privilege mode (read): 0 = counters are stopped while in debug mode 1 = counters are running while in debug mode Privilege mode (write): 0 = stop counters in debug mode 1 = continue counting in debug mode"]
pub type CosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NTUSEL` reader - 19:16\\]
These bits determine which NTU input signal is used as external timebase. There are up to four inputs supported with four valid selection combinations. Any invalid selection value written to the NTUSEL bit-field will result in a TIED LOW being used as the NTU signal. The NTU signal will also be TIED LOW in case of a single-bit flip as it will result in an invalid combination of NTUSEL. User and privilege mode (read): 0000 = NTU0 0101 = NTU1 1010 = NTU2 1111 = NTU3 other = tied to '0' Privilege mode (write): 0000 = NTU0 0101 = NTU1 1010 = NTU2 1111 = NTU3 other = tied to '0'"]
pub type NtuselR = crate::FieldReader;
#[doc = "Field `NTUSEL` writer - 19:16\\]
These bits determine which NTU input signal is used as external timebase. There are up to four inputs supported with four valid selection combinations. Any invalid selection value written to the NTUSEL bit-field will result in a TIED LOW being used as the NTU signal. The NTU signal will also be TIED LOW in case of a single-bit flip as it will result in an invalid combination of NTUSEL. User and privilege mode (read): 0000 = NTU0 0101 = NTU1 1010 = NTU2 1111 = NTU3 other = tied to '0' Privilege mode (write): 0000 = NTU0 0101 = NTU1 1010 = NTU2 1111 = NTU3 other = tied to '0'"]
pub type NtuselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
The CNT0EN bit starts and stops the operation of counter block 0 (UC0 and FRC0). User and privilege mode (read): 0 = counters are stopped 1 = counters are running Privilege mode (write): 0 = stop counters 1 = start counters"]
    #[inline(always)]
    pub fn cnt0en(&self) -> Cnt0enR {
        Cnt0enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
The CNT1EN bit starts and stops the operation of counter block 1 (UC1 and FRC1). User and privilege mode (read): 0 = counters are stopped 1 = counters are running Privilege mode (write): 0 = stop counters 1 = start counters"]
    #[inline(always)]
    pub fn cnt1en(&self) -> Cnt1enR {
        Cnt1enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
This bit determines if both counters are stopped when the device goes into debug mode or if they continue counting. User and privilege mode (read): 0 = counters are stopped while in debug mode 1 = counters are running while in debug mode Privilege mode (write): 0 = stop counters in debug mode 1 = continue counting in debug mode"]
    #[inline(always)]
    pub fn cos(&self) -> CosR {
        CosR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - 19:16\\]
These bits determine which NTU input signal is used as external timebase. There are up to four inputs supported with four valid selection combinations. Any invalid selection value written to the NTUSEL bit-field will result in a TIED LOW being used as the NTU signal. The NTU signal will also be TIED LOW in case of a single-bit flip as it will result in an invalid combination of NTUSEL. User and privilege mode (read): 0000 = NTU0 0101 = NTU1 1010 = NTU2 1111 = NTU3 other = tied to '0' Privilege mode (write): 0000 = NTU0 0101 = NTU1 1010 = NTU2 1111 = NTU3 other = tied to '0'"]
    #[inline(always)]
    pub fn ntusel(&self) -> NtuselR {
        NtuselR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
The CNT0EN bit starts and stops the operation of counter block 0 (UC0 and FRC0). User and privilege mode (read): 0 = counters are stopped 1 = counters are running Privilege mode (write): 0 = stop counters 1 = start counters"]
    #[inline(always)]
    #[must_use]
    pub fn cnt0en(&mut self) -> Cnt0enW<CfgGctrlSpec> {
        Cnt0enW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
The CNT1EN bit starts and stops the operation of counter block 1 (UC1 and FRC1). User and privilege mode (read): 0 = counters are stopped 1 = counters are running Privilege mode (write): 0 = stop counters 1 = start counters"]
    #[inline(always)]
    #[must_use]
    pub fn cnt1en(&mut self) -> Cnt1enW<CfgGctrlSpec> {
        Cnt1enW::new(self, 1)
    }
    #[doc = "Bit 15 - 15:15\\]
This bit determines if both counters are stopped when the device goes into debug mode or if they continue counting. User and privilege mode (read): 0 = counters are stopped while in debug mode 1 = counters are running while in debug mode Privilege mode (write): 0 = stop counters in debug mode 1 = continue counting in debug mode"]
    #[inline(always)]
    #[must_use]
    pub fn cos(&mut self) -> CosW<CfgGctrlSpec> {
        CosW::new(self, 15)
    }
    #[doc = "Bits 16:19 - 19:16\\]
These bits determine which NTU input signal is used as external timebase. There are up to four inputs supported with four valid selection combinations. Any invalid selection value written to the NTUSEL bit-field will result in a TIED LOW being used as the NTU signal. The NTU signal will also be TIED LOW in case of a single-bit flip as it will result in an invalid combination of NTUSEL. User and privilege mode (read): 0000 = NTU0 0101 = NTU1 1010 = NTU2 1111 = NTU3 other = tied to '0' Privilege mode (write): 0000 = NTU0 0101 = NTU1 1010 = NTU2 1111 = NTU3 other = tied to '0'"]
    #[inline(always)]
    #[must_use]
    pub fn ntusel(&mut self) -> NtuselW<CfgGctrlSpec> {
        NtuselW::new(self, 16)
    }
}
#[doc = "CFG_GCTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgGctrlSpec;
impl crate::RegisterSpec for CfgGctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_gctrl::R`](R) reader structure"]
impl crate::Readable for CfgGctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_gctrl::W`](W) writer structure"]
impl crate::Writable for CfgGctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_GCTRL to value 0"]
impl crate::Resettable for CfgGctrlSpec {
    const RESET_VALUE: u32 = 0;
}
