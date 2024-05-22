#[doc = "Register `CTI__CFG__CSCTI_CFG_LSR` reader"]
pub type R = crate::R<Cti_Cfg_CsctiCfgLsrSpec>;
#[doc = "Register `CTI__CFG__CSCTI_CFG_LSR` writer"]
pub type W = crate::W<Cti_Cfg_CsctiCfgLsrSpec>;
#[doc = "Field `LOCKEXIST` reader - 0:0\\]
Indicates that a lock control mechanism exists for this device. This bit reads as 0 when read from an external debugger (paddrdbg31 = 1) since external debugger accesses are not subject to Lock Registers."]
pub type LockexistR = crate::BitReader;
#[doc = "Field `LOCKEXIST` writer - 0:0\\]
Indicates that a lock control mechanism exists for this device. This bit reads as 0 when read from an external debugger (paddrdbg31 = 1) since external debugger accesses are not subject to Lock Registers."]
pub type LockexistW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCKGRANT` reader - 1:1\\]
Returns the current status of the Lock. This bit reads as 0 when read from an external debugger (paddrdbg31 = 1) since external debugger accesses are not subject to Lock Registers."]
pub type LockgrantR = crate::BitReader;
#[doc = "Field `LOCKGRANT` writer - 1:1\\]
Returns the current status of the Lock. This bit reads as 0 when read from an external debugger (paddrdbg31 = 1) since external debugger accesses are not subject to Lock Registers."]
pub type LockgrantW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCKTYPE` reader - 2:2\\]
Indicates if the Lock Access Register (0xFB0) is implemented as 8-bit or 32-bit"]
pub type LocktypeR = crate::BitReader;
#[doc = "Field `LOCKTYPE` writer - 2:2\\]
Indicates if the Lock Access Register (0xFB0) is implemented as 8-bit or 32-bit"]
pub type LocktypeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Indicates that a lock control mechanism exists for this device. This bit reads as 0 when read from an external debugger (paddrdbg31 = 1) since external debugger accesses are not subject to Lock Registers."]
    #[inline(always)]
    pub fn lockexist(&self) -> LockexistR {
        LockexistR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Returns the current status of the Lock. This bit reads as 0 when read from an external debugger (paddrdbg31 = 1) since external debugger accesses are not subject to Lock Registers."]
    #[inline(always)]
    pub fn lockgrant(&self) -> LockgrantR {
        LockgrantR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Indicates if the Lock Access Register (0xFB0) is implemented as 8-bit or 32-bit"]
    #[inline(always)]
    pub fn locktype(&self) -> LocktypeR {
        LocktypeR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Indicates that a lock control mechanism exists for this device. This bit reads as 0 when read from an external debugger (paddrdbg31 = 1) since external debugger accesses are not subject to Lock Registers."]
    #[inline(always)]
    #[must_use]
    pub fn lockexist(&mut self) -> LockexistW<Cti_Cfg_CsctiCfgLsrSpec> {
        LockexistW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Returns the current status of the Lock. This bit reads as 0 when read from an external debugger (paddrdbg31 = 1) since external debugger accesses are not subject to Lock Registers."]
    #[inline(always)]
    #[must_use]
    pub fn lockgrant(&mut self) -> LockgrantW<Cti_Cfg_CsctiCfgLsrSpec> {
        LockgrantW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Indicates if the Lock Access Register (0xFB0) is implemented as 8-bit or 32-bit"]
    #[inline(always)]
    #[must_use]
    pub fn locktype(&mut self) -> LocktypeW<Cti_Cfg_CsctiCfgLsrSpec> {
        LocktypeW::new(self, 2)
    }
}
#[doc = "This indicates the status of the Lock control mechanism. This lock prevents accidental writes by code under debug. When locked, write access is blocked to all registers, except the Lock Access Register. External accesses from a debugger (paddrdbg31 = 1) are not subject to the Lock Registers. This register reads as 0 when read from an external debugger (paddrdbg31 = 1).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_lsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_lsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cti_Cfg_CsctiCfgLsrSpec;
impl crate::RegisterSpec for Cti_Cfg_CsctiCfgLsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cti__cfg__cscti_cfg_lsr::R`](R) reader structure"]
impl crate::Readable for Cti_Cfg_CsctiCfgLsrSpec {}
#[doc = "`write(|w| ..)` method takes [`cti__cfg__cscti_cfg_lsr::W`](W) writer structure"]
impl crate::Writable for Cti_Cfg_CsctiCfgLsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTI__CFG__CSCTI_CFG_LSR to value 0x03"]
impl crate::Resettable for Cti_Cfg_CsctiCfgLsrSpec {
    const RESET_VALUE: u32 = 0x03;
}
