#[doc = "Register `APBADDR_DBG_CPU0_DBGWCR0_EL1` reader"]
pub type R = crate::R<ApbaddrDbgCpu0Dbgwcr0El1Spec>;
#[doc = "Register `APBADDR_DBG_CPU0_DBGWCR0_EL1` writer"]
pub type W = crate::W<ApbaddrDbgCpu0Dbgwcr0El1Spec>;
#[doc = "Field `E` reader - 0:0\\]
Enable watchpoint n. Possible values are: 0 Watchpoint disabled. 1 Watchpoint enabled."]
pub type ER = crate::BitReader;
#[doc = "Field `E` writer - 0:0\\]
Enable watchpoint n. Possible values are: 0 Watchpoint disabled. 1 Watchpoint enabled."]
pub type EW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAC` reader - 2:1\\]
Privilege of access control. Determines the exception level or levels at which a watchpoint debug event for watchpoint n is generated. This field must be interpreted along with the SSC and HMC fields."]
pub type PacR = crate::FieldReader;
#[doc = "Field `PAC` writer - 2:1\\]
Privilege of access control. Determines the exception level or levels at which a watchpoint debug event for watchpoint n is generated. This field must be interpreted along with the SSC and HMC fields."]
pub type PacW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LSC` reader - 4:3\\]
Load/store control. This field enables watchpoint matching on the type of access being made. Possible values of this field are: 01 Match instructions that load from a watchpointed address. 10 Match instructions that store to a watchpointed address. 11 Match instructions that load from or store to a watchpointed address. All other values are reserved, but must behave as if the watchpoint is disabled. Software must not rely on this property as the behavior of reserved values might change in a future revision of the architecture.Ignored if E is 0."]
pub type LscR = crate::FieldReader;
#[doc = "Field `LSC` writer - 4:3\\]
Load/store control. This field enables watchpoint matching on the type of access being made. Possible values of this field are: 01 Match instructions that load from a watchpointed address. 10 Match instructions that store to a watchpointed address. 11 Match instructions that load from or store to a watchpointed address. All other values are reserved, but must behave as if the watchpoint is disabled. Software must not rely on this property as the behavior of reserved values might change in a future revision of the architecture.Ignored if E is 0."]
pub type LscW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BAS` reader - 12:5\\]
Byte address select. Each bit of this field selects whether a byte from within the word or double-word addressed by DBGWVR&lt;n>_EL1 is being watched.BASDescriptionxxxxxxx1Match byte at DBGWVR&lt;n>_EL1xxxxxx1xMatch byte at DBGWVR&lt;n>_EL1+1xxxxx1xxMatch byte at DBGWVR&lt;n>_EL1+2xxxx1xxxMatch byte at DBGWVR&lt;n>_EL1+3In cases where DBGWVR&lt;n>_EL1 addresses a double-word:BASDescription, if DBGWVR&lt;n>_EL1\\[2\\]
== 0xxx1xxxxMatch byte at DBGWVR&lt;n>_EL1+4xx1xxxxxMatch byte at DBGWVR&lt;n>_EL1+5x1xxxxxxMatch byte at DBGWVR&lt;n>_EL1+61xxxxxxxMatch byte at DBGWVR&lt;n>_EL1+7If DBGWVR&lt;n>_EL1\\[2\\]
== 1, only BAS\\[3:0\\]
is used. ARM deprecates setting DBGWVR&lt;n>_EL1 == 1.The valid values for BAS are 0b0000000, or a binary number all of whose set bits are contiguous. All other values are reserved and must not be used by software.If BAS is zero, no bytes are watched by this watchpoint.Ignored if E is 0."]
pub type BasR = crate::FieldReader;
#[doc = "Field `BAS` writer - 12:5\\]
Byte address select. Each bit of this field selects whether a byte from within the word or double-word addressed by DBGWVR&lt;n>_EL1 is being watched.BASDescriptionxxxxxxx1Match byte at DBGWVR&lt;n>_EL1xxxxxx1xMatch byte at DBGWVR&lt;n>_EL1+1xxxxx1xxMatch byte at DBGWVR&lt;n>_EL1+2xxxx1xxxMatch byte at DBGWVR&lt;n>_EL1+3In cases where DBGWVR&lt;n>_EL1 addresses a double-word:BASDescription, if DBGWVR&lt;n>_EL1\\[2\\]
== 0xxx1xxxxMatch byte at DBGWVR&lt;n>_EL1+4xx1xxxxxMatch byte at DBGWVR&lt;n>_EL1+5x1xxxxxxMatch byte at DBGWVR&lt;n>_EL1+61xxxxxxxMatch byte at DBGWVR&lt;n>_EL1+7If DBGWVR&lt;n>_EL1\\[2\\]
== 1, only BAS\\[3:0\\]
is used. ARM deprecates setting DBGWVR&lt;n>_EL1 == 1.The valid values for BAS are 0b0000000, or a binary number all of whose set bits are contiguous. All other values are reserved and must not be used by software.If BAS is zero, no bytes are watched by this watchpoint.Ignored if E is 0."]
pub type BasW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HMC` reader - 13:13\\]
Higher mode control. Determines the debug perspective for deciding when a watchpoint debug event for watchpoint n is generated. This field must be interpreted along with the SSC and PAC fields."]
pub type HmcR = crate::BitReader;
#[doc = "Field `HMC` writer - 13:13\\]
Higher mode control. Determines the debug perspective for deciding when a watchpoint debug event for watchpoint n is generated. This field must be interpreted along with the SSC and PAC fields."]
pub type HmcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSC` reader - 15:14\\]
Security state control. Determines the security states under which a watchpoint debug event for watchpoint n is generated. This field must be interpreted along with the HMC and PAC fields."]
pub type SscR = crate::FieldReader;
#[doc = "Field `SSC` writer - 15:14\\]
Security state control. Determines the security states under which a watchpoint debug event for watchpoint n is generated. This field must be interpreted along with the HMC and PAC fields."]
pub type SscW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LBN` reader - 19:16\\]
Linked breakpoint number. For Linked data address watchpoints, this specifies the index of the Context-matching breakpoint linked to."]
pub type LbnR = crate::FieldReader;
#[doc = "Field `LBN` writer - 19:16\\]
Linked breakpoint number. For Linked data address watchpoints, this specifies the index of the Context-matching breakpoint linked to."]
pub type LbnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WT` reader - 20:20\\]
Watchpoint type. Possible values are: 0 Unlinked data address match. 1 Linked data address match."]
pub type WtR = crate::BitReader;
#[doc = "Field `WT` writer - 20:20\\]
Watchpoint type. Possible values are: 0 Unlinked data address match. 1 Linked data address match."]
pub type WtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES0_DBGWCR0_EL1_23_21` reader - 23:21\\]
Reserved, RES0."]
pub type Res0Dbgwcr0El1_23_21R = crate::FieldReader;
#[doc = "Field `RES0_DBGWCR0_EL1_23_21` writer - 23:21\\]
Reserved, RES0."]
pub type Res0Dbgwcr0El1_23_21W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MASK` reader - 28:24\\]
Address mask. Only objects up to 2GB can be watched using a single mask. 00000 No mask. 00001 Reserved. 00010 Reserved. Other values mask the corresponding number of address bits, from 0b00011 masking 3 address bits \\[0x00000007 mask for address\\]
to 0b11111 masking 31 address bits \\[0x7FFFFFFF mask for address\\]."]
pub type MaskR = crate::FieldReader;
#[doc = "Field `MASK` writer - 28:24\\]
Address mask. Only objects up to 2GB can be watched using a single mask. 00000 No mask. 00001 Reserved. 00010 Reserved. Other values mask the corresponding number of address bits, from 0b00011 masking 3 address bits \\[0x00000007 mask for address\\]
to 0b11111 masking 31 address bits \\[0x7FFFFFFF mask for address\\]."]
pub type MaskW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RES0_DBGWCR0_EL1_31_29` reader - 31:29\\]
Reserved, RES0."]
pub type Res0Dbgwcr0El1_31_29R = crate::FieldReader;
#[doc = "Field `RES0_DBGWCR0_EL1_31_29` writer - 31:29\\]
Reserved, RES0."]
pub type Res0Dbgwcr0El1_31_29W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable watchpoint n. Possible values are: 0 Watchpoint disabled. 1 Watchpoint enabled."]
    #[inline(always)]
    pub fn e(&self) -> ER {
        ER::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Privilege of access control. Determines the exception level or levels at which a watchpoint debug event for watchpoint n is generated. This field must be interpreted along with the SSC and HMC fields."]
    #[inline(always)]
    pub fn pac(&self) -> PacR {
        PacR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - 4:3\\]
Load/store control. This field enables watchpoint matching on the type of access being made. Possible values of this field are: 01 Match instructions that load from a watchpointed address. 10 Match instructions that store to a watchpointed address. 11 Match instructions that load from or store to a watchpointed address. All other values are reserved, but must behave as if the watchpoint is disabled. Software must not rely on this property as the behavior of reserved values might change in a future revision of the architecture.Ignored if E is 0."]
    #[inline(always)]
    pub fn lsc(&self) -> LscR {
        LscR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:12 - 12:5\\]
Byte address select. Each bit of this field selects whether a byte from within the word or double-word addressed by DBGWVR&lt;n>_EL1 is being watched.BASDescriptionxxxxxxx1Match byte at DBGWVR&lt;n>_EL1xxxxxx1xMatch byte at DBGWVR&lt;n>_EL1+1xxxxx1xxMatch byte at DBGWVR&lt;n>_EL1+2xxxx1xxxMatch byte at DBGWVR&lt;n>_EL1+3In cases where DBGWVR&lt;n>_EL1 addresses a double-word:BASDescription, if DBGWVR&lt;n>_EL1\\[2\\]
== 0xxx1xxxxMatch byte at DBGWVR&lt;n>_EL1+4xx1xxxxxMatch byte at DBGWVR&lt;n>_EL1+5x1xxxxxxMatch byte at DBGWVR&lt;n>_EL1+61xxxxxxxMatch byte at DBGWVR&lt;n>_EL1+7If DBGWVR&lt;n>_EL1\\[2\\]
== 1, only BAS\\[3:0\\]
is used. ARM deprecates setting DBGWVR&lt;n>_EL1 == 1.The valid values for BAS are 0b0000000, or a binary number all of whose set bits are contiguous. All other values are reserved and must not be used by software.If BAS is zero, no bytes are watched by this watchpoint.Ignored if E is 0."]
    #[inline(always)]
    pub fn bas(&self) -> BasR {
        BasR::new(((self.bits >> 5) & 0xff) as u8)
    }
    #[doc = "Bit 13 - 13:13\\]
Higher mode control. Determines the debug perspective for deciding when a watchpoint debug event for watchpoint n is generated. This field must be interpreted along with the SSC and PAC fields."]
    #[inline(always)]
    pub fn hmc(&self) -> HmcR {
        HmcR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Security state control. Determines the security states under which a watchpoint debug event for watchpoint n is generated. This field must be interpreted along with the HMC and PAC fields."]
    #[inline(always)]
    pub fn ssc(&self) -> SscR {
        SscR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Linked breakpoint number. For Linked data address watchpoints, this specifies the index of the Context-matching breakpoint linked to."]
    #[inline(always)]
    pub fn lbn(&self) -> LbnR {
        LbnR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - 20:20\\]
Watchpoint type. Possible values are: 0 Unlinked data address match. 1 Linked data address match."]
    #[inline(always)]
    pub fn wt(&self) -> WtR {
        WtR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:23 - 23:21\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_dbgwcr0_el1_23_21(&self) -> Res0Dbgwcr0El1_23_21R {
        Res0Dbgwcr0El1_23_21R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Address mask. Only objects up to 2GB can be watched using a single mask. 00000 No mask. 00001 Reserved. 00010 Reserved. Other values mask the corresponding number of address bits, from 0b00011 masking 3 address bits \\[0x00000007 mask for address\\]
to 0b11111 masking 31 address bits \\[0x7FFFFFFF mask for address\\]."]
    #[inline(always)]
    pub fn mask(&self) -> MaskR {
        MaskR::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 29:31 - 31:29\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_dbgwcr0_el1_31_29(&self) -> Res0Dbgwcr0El1_31_29R {
        Res0Dbgwcr0El1_31_29R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enable watchpoint n. Possible values are: 0 Watchpoint disabled. 1 Watchpoint enabled."]
    #[inline(always)]
    #[must_use]
    pub fn e(&mut self) -> EW<ApbaddrDbgCpu0Dbgwcr0El1Spec> {
        EW::new(self, 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Privilege of access control. Determines the exception level or levels at which a watchpoint debug event for watchpoint n is generated. This field must be interpreted along with the SSC and HMC fields."]
    #[inline(always)]
    #[must_use]
    pub fn pac(&mut self) -> PacW<ApbaddrDbgCpu0Dbgwcr0El1Spec> {
        PacW::new(self, 1)
    }
    #[doc = "Bits 3:4 - 4:3\\]
Load/store control. This field enables watchpoint matching on the type of access being made. Possible values of this field are: 01 Match instructions that load from a watchpointed address. 10 Match instructions that store to a watchpointed address. 11 Match instructions that load from or store to a watchpointed address. All other values are reserved, but must behave as if the watchpoint is disabled. Software must not rely on this property as the behavior of reserved values might change in a future revision of the architecture.Ignored if E is 0."]
    #[inline(always)]
    #[must_use]
    pub fn lsc(&mut self) -> LscW<ApbaddrDbgCpu0Dbgwcr0El1Spec> {
        LscW::new(self, 3)
    }
    #[doc = "Bits 5:12 - 12:5\\]
Byte address select. Each bit of this field selects whether a byte from within the word or double-word addressed by DBGWVR&lt;n>_EL1 is being watched.BASDescriptionxxxxxxx1Match byte at DBGWVR&lt;n>_EL1xxxxxx1xMatch byte at DBGWVR&lt;n>_EL1+1xxxxx1xxMatch byte at DBGWVR&lt;n>_EL1+2xxxx1xxxMatch byte at DBGWVR&lt;n>_EL1+3In cases where DBGWVR&lt;n>_EL1 addresses a double-word:BASDescription, if DBGWVR&lt;n>_EL1\\[2\\]
== 0xxx1xxxxMatch byte at DBGWVR&lt;n>_EL1+4xx1xxxxxMatch byte at DBGWVR&lt;n>_EL1+5x1xxxxxxMatch byte at DBGWVR&lt;n>_EL1+61xxxxxxxMatch byte at DBGWVR&lt;n>_EL1+7If DBGWVR&lt;n>_EL1\\[2\\]
== 1, only BAS\\[3:0\\]
is used. ARM deprecates setting DBGWVR&lt;n>_EL1 == 1.The valid values for BAS are 0b0000000, or a binary number all of whose set bits are contiguous. All other values are reserved and must not be used by software.If BAS is zero, no bytes are watched by this watchpoint.Ignored if E is 0."]
    #[inline(always)]
    #[must_use]
    pub fn bas(&mut self) -> BasW<ApbaddrDbgCpu0Dbgwcr0El1Spec> {
        BasW::new(self, 5)
    }
    #[doc = "Bit 13 - 13:13\\]
Higher mode control. Determines the debug perspective for deciding when a watchpoint debug event for watchpoint n is generated. This field must be interpreted along with the SSC and PAC fields."]
    #[inline(always)]
    #[must_use]
    pub fn hmc(&mut self) -> HmcW<ApbaddrDbgCpu0Dbgwcr0El1Spec> {
        HmcW::new(self, 13)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Security state control. Determines the security states under which a watchpoint debug event for watchpoint n is generated. This field must be interpreted along with the HMC and PAC fields."]
    #[inline(always)]
    #[must_use]
    pub fn ssc(&mut self) -> SscW<ApbaddrDbgCpu0Dbgwcr0El1Spec> {
        SscW::new(self, 14)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Linked breakpoint number. For Linked data address watchpoints, this specifies the index of the Context-matching breakpoint linked to."]
    #[inline(always)]
    #[must_use]
    pub fn lbn(&mut self) -> LbnW<ApbaddrDbgCpu0Dbgwcr0El1Spec> {
        LbnW::new(self, 16)
    }
    #[doc = "Bit 20 - 20:20\\]
Watchpoint type. Possible values are: 0 Unlinked data address match. 1 Linked data address match."]
    #[inline(always)]
    #[must_use]
    pub fn wt(&mut self) -> WtW<ApbaddrDbgCpu0Dbgwcr0El1Spec> {
        WtW::new(self, 20)
    }
    #[doc = "Bits 21:23 - 23:21\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_dbgwcr0_el1_23_21(
        &mut self,
    ) -> Res0Dbgwcr0El1_23_21W<ApbaddrDbgCpu0Dbgwcr0El1Spec> {
        Res0Dbgwcr0El1_23_21W::new(self, 21)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Address mask. Only objects up to 2GB can be watched using a single mask. 00000 No mask. 00001 Reserved. 00010 Reserved. Other values mask the corresponding number of address bits, from 0b00011 masking 3 address bits \\[0x00000007 mask for address\\]
to 0b11111 masking 31 address bits \\[0x7FFFFFFF mask for address\\]."]
    #[inline(always)]
    #[must_use]
    pub fn mask(&mut self) -> MaskW<ApbaddrDbgCpu0Dbgwcr0El1Spec> {
        MaskW::new(self, 24)
    }
    #[doc = "Bits 29:31 - 31:29\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_dbgwcr0_el1_31_29(
        &mut self,
    ) -> Res0Dbgwcr0El1_31_29W<ApbaddrDbgCpu0Dbgwcr0El1Spec> {
        Res0Dbgwcr0El1_31_29W::new(self, 29)
    }
}
#[doc = "Debug Watchpoint Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_dbgwcr0_el1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_dbgwcr0_el1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrDbgCpu0Dbgwcr0El1Spec;
impl crate::RegisterSpec for ApbaddrDbgCpu0Dbgwcr0El1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_dbg_cpu0_dbgwcr0_el1::R`](R) reader structure"]
impl crate::Readable for ApbaddrDbgCpu0Dbgwcr0El1Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_dbg_cpu0_dbgwcr0_el1::W`](W) writer structure"]
impl crate::Writable for ApbaddrDbgCpu0Dbgwcr0El1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_DBG_CPU0_DBGWCR0_EL1 to value 0"]
impl crate::Resettable for ApbaddrDbgCpu0Dbgwcr0El1Spec {
    const RESET_VALUE: u32 = 0;
}
