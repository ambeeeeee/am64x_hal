#[doc = "Register `CFG0_FSS_CTRL` reader"]
pub type R = crate::R<Cfg0FssCtrlSpec>;
#[doc = "Register `CFG0_FSS_CTRL` writer"]
pub type W = crate::W<Cfg0FssCtrlSpec>;
#[doc = "Field `FSS_CTRL_S0_BOOT_SEG` reader - 5:0\\]
Selects the boot block to be used for the S0 (OSPI0) flash interface. If the s0_boot_size is 128 MB then only bits \\[4:0\\]
of this field are used. Care must be taken to account for the address translation as to not fall off or wrap the address of the flash. (e.g. if both ECC and authentication are activated for 64 MB boot, the highest valid block number is is 49, as sector 50 is only .2M Bytes in size.)"]
pub type FssCtrlS0BootSegR = crate::FieldReader;
#[doc = "Field `FSS_CTRL_S0_BOOT_SEG` writer - 5:0\\]
Selects the boot block to be used for the S0 (OSPI0) flash interface. If the s0_boot_size is 128 MB then only bits \\[4:0\\]
of this field are used. Care must be taken to account for the address translation as to not fall off or wrap the address of the flash. (e.g. if both ECC and authentication are activated for 64 MB boot, the highest valid block number is is 49, as sector 50 is only .2M Bytes in size.)"]
pub type FssCtrlS0BootSegW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `FSS_CTRL_S0_BOOT_SIZE` reader - 8:8\\]
Selects the size of the boot block to be used for the S0 (OSPI0) flash interface"]
pub type FssCtrlS0BootSizeR = crate::BitReader;
#[doc = "Field `FSS_CTRL_S0_BOOT_SIZE` writer - 8:8\\]
Selects the size of the boot block to be used for the S0 (OSPI0) flash interface"]
pub type FssCtrlS0BootSizeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Selects the boot block to be used for the S0 (OSPI0) flash interface. If the s0_boot_size is 128 MB then only bits \\[4:0\\]
of this field are used. Care must be taken to account for the address translation as to not fall off or wrap the address of the flash. (e.g. if both ECC and authentication are activated for 64 MB boot, the highest valid block number is is 49, as sector 50 is only .2M Bytes in size.)"]
    #[inline(always)]
    pub fn fss_ctrl_s0_boot_seg(&self) -> FssCtrlS0BootSegR {
        FssCtrlS0BootSegR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Selects the size of the boot block to be used for the S0 (OSPI0) flash interface"]
    #[inline(always)]
    pub fn fss_ctrl_s0_boot_size(&self) -> FssCtrlS0BootSizeR {
        FssCtrlS0BootSizeR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Selects the boot block to be used for the S0 (OSPI0) flash interface. If the s0_boot_size is 128 MB then only bits \\[4:0\\]
of this field are used. Care must be taken to account for the address translation as to not fall off or wrap the address of the flash. (e.g. if both ECC and authentication are activated for 64 MB boot, the highest valid block number is is 49, as sector 50 is only .2M Bytes in size.)"]
    #[inline(always)]
    #[must_use]
    pub fn fss_ctrl_s0_boot_seg(&mut self) -> FssCtrlS0BootSegW<Cfg0FssCtrlSpec> {
        FssCtrlS0BootSegW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Selects the size of the boot block to be used for the S0 (OSPI0) flash interface"]
    #[inline(always)]
    #[must_use]
    pub fn fss_ctrl_s0_boot_size(&mut self) -> FssCtrlS0BootSizeW<Cfg0FssCtrlSpec> {
        FssCtrlS0BootSizeW::new(self, 8)
    }
}
#[doc = "CFG0_FSS_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_fss_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_fss_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0FssCtrlSpec;
impl crate::RegisterSpec for Cfg0FssCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_fss_ctrl::R`](R) reader structure"]
impl crate::Readable for Cfg0FssCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_fss_ctrl::W`](W) writer structure"]
impl crate::Writable for Cfg0FssCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_FSS_CTRL to value 0"]
impl crate::Resettable for Cfg0FssCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
