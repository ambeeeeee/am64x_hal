#[doc = "Register `CFG_TX_LOCK_CTRL` reader"]
pub type R = crate::R<CfgTxLockCtrlSpec>;
#[doc = "Register `CFG_TX_LOCK_CTRL` writer"]
pub type W = crate::W<CfgTxLockCtrlSpec>;
#[doc = "Field `LOCK` reader - 0:0\\]
Control Register Lock Enable bitThis bit locks the contents of all the transmit control registers that support a lock protection. Once locked, further writes will not take effect until a SYSRS has reset this register. Once set, further writes to this bit will be ignored. 0h \\[R/W\\]
= Transmit control registers can be modified and are not locked.1h \\[R/W\\]
= Transmit control registers are locked and cannot be modified until this bit is cleared by SYSRS. Any further writes to this bit are ignored. Note: The KEY field must contatin 0xA5 for any write to this bit to take effect."]
pub type LockR = crate::BitReader;
#[doc = "Field `LOCK` writer - 0:0\\]
Control Register Lock Enable bitThis bit locks the contents of all the transmit control registers that support a lock protection. Once locked, further writes will not take effect until a SYSRS has reset this register. Once set, further writes to this bit will be ignored. 0h \\[R/W\\]
= Transmit control registers can be modified and are not locked.1h \\[R/W\\]
= Transmit control registers are locked and cannot be modified until this bit is cleared by SYSRS. Any further writes to this bit are ignored. Note: The KEY field must contatin 0xA5 for any write to this bit to take effect."]
pub type LockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEY` reader - 15:8\\]
Write KeyIn order to write to this register, 0xA5 must be written to this field at the same time. Otherwise, writes are ignored. The key is cleared immediately after writing, so it must be written again for every change to this register."]
pub type KeyR = crate::FieldReader;
#[doc = "Field `KEY` writer - 15:8\\]
Write KeyIn order to write to this register, 0xA5 must be written to this field at the same time. Otherwise, writes are ignored. The key is cleared immediately after writing, so it must be written again for every change to this register."]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Control Register Lock Enable bitThis bit locks the contents of all the transmit control registers that support a lock protection. Once locked, further writes will not take effect until a SYSRS has reset this register. Once set, further writes to this bit will be ignored. 0h \\[R/W\\]
= Transmit control registers can be modified and are not locked.1h \\[R/W\\]
= Transmit control registers are locked and cannot be modified until this bit is cleared by SYSRS. Any further writes to this bit are ignored. Note: The KEY field must contatin 0xA5 for any write to this bit to take effect."]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Write KeyIn order to write to this register, 0xA5 must be written to this field at the same time. Otherwise, writes are ignored. The key is cleared immediately after writing, so it must be written again for every change to this register."]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Control Register Lock Enable bitThis bit locks the contents of all the transmit control registers that support a lock protection. Once locked, further writes will not take effect until a SYSRS has reset this register. Once set, further writes to this bit will be ignored. 0h \\[R/W\\]
= Transmit control registers can be modified and are not locked.1h \\[R/W\\]
= Transmit control registers are locked and cannot be modified until this bit is cleared by SYSRS. Any further writes to this bit are ignored. Note: The KEY field must contatin 0xA5 for any write to this bit to take effect."]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LockW<CfgTxLockCtrlSpec> {
        LockW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Write KeyIn order to write to this register, 0xA5 must be written to this field at the same time. Otherwise, writes are ignored. The key is cleared immediately after writing, so it must be written again for every change to this register."]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KeyW<CfgTxLockCtrlSpec> {
        KeyW::new(self, 8)
    }
}
#[doc = "Transmit lock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tx_lock_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tx_lock_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgTxLockCtrlSpec;
impl crate::RegisterSpec for CfgTxLockCtrlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cfg_tx_lock_ctrl::R`](R) reader structure"]
impl crate::Readable for CfgTxLockCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_tx_lock_ctrl::W`](W) writer structure"]
impl crate::Writable for CfgTxLockCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CFG_TX_LOCK_CTRL to value 0"]
impl crate::Resettable for CfgTxLockCtrlSpec {
    const RESET_VALUE: u16 = 0;
}
