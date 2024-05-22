#[doc = "Register `CFG0_FSS_CTRL_PROXY` reader"]
pub type R = crate::R<Cfg0FssCtrlProxySpec>;
#[doc = "Register `CFG0_FSS_CTRL_PROXY` writer"]
pub type W = crate::W<Cfg0FssCtrlProxySpec>;
#[doc = "Field `FSS_CTRL_S0_BOOT_SEG_PROXY` reader - 5:0\\]
Selects the boot block to be used for the S0 (OSPI0) flash interface. If the s0_boot_size is 128 MB then only bits \\[4:0\\]
of this field are used. Care must be taken to account for the address translation as to not fall off or wrap the address of the flash. (e.g. if both ECC and authentication are activated for 64 MB boot, the highest valid block number is is 49, as sector 50 is only .2M Bytes in size.)"]
pub type FssCtrlS0BootSegProxyR = crate::FieldReader;
#[doc = "Field `FSS_CTRL_S0_BOOT_SEG_PROXY` writer - 5:0\\]
Selects the boot block to be used for the S0 (OSPI0) flash interface. If the s0_boot_size is 128 MB then only bits \\[4:0\\]
of this field are used. Care must be taken to account for the address translation as to not fall off or wrap the address of the flash. (e.g. if both ECC and authentication are activated for 64 MB boot, the highest valid block number is is 49, as sector 50 is only .2M Bytes in size.)"]
pub type FssCtrlS0BootSegProxyW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `FSS_CTRL_S0_BOOT_SIZE_PROXY` reader - 8:8\\]
Selects the size of the boot block to be used for the S0 (OSPI0) flash interface"]
pub type FssCtrlS0BootSizeProxyR = crate::BitReader;
#[doc = "Field `FSS_CTRL_S0_BOOT_SIZE_PROXY` writer - 8:8\\]
Selects the size of the boot block to be used for the S0 (OSPI0) flash interface"]
pub type FssCtrlS0BootSizeProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Selects the boot block to be used for the S0 (OSPI0) flash interface. If the s0_boot_size is 128 MB then only bits \\[4:0\\]
of this field are used. Care must be taken to account for the address translation as to not fall off or wrap the address of the flash. (e.g. if both ECC and authentication are activated for 64 MB boot, the highest valid block number is is 49, as sector 50 is only .2M Bytes in size.)"]
    #[inline(always)]
    pub fn fss_ctrl_s0_boot_seg_proxy(&self) -> FssCtrlS0BootSegProxyR {
        FssCtrlS0BootSegProxyR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Selects the size of the boot block to be used for the S0 (OSPI0) flash interface"]
    #[inline(always)]
    pub fn fss_ctrl_s0_boot_size_proxy(&self) -> FssCtrlS0BootSizeProxyR {
        FssCtrlS0BootSizeProxyR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Selects the boot block to be used for the S0 (OSPI0) flash interface. If the s0_boot_size is 128 MB then only bits \\[4:0\\]
of this field are used. Care must be taken to account for the address translation as to not fall off or wrap the address of the flash. (e.g. if both ECC and authentication are activated for 64 MB boot, the highest valid block number is is 49, as sector 50 is only .2M Bytes in size.)"]
    #[inline(always)]
    #[must_use]
    pub fn fss_ctrl_s0_boot_seg_proxy(&mut self) -> FssCtrlS0BootSegProxyW<Cfg0FssCtrlProxySpec> {
        FssCtrlS0BootSegProxyW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Selects the size of the boot block to be used for the S0 (OSPI0) flash interface"]
    #[inline(always)]
    #[must_use]
    pub fn fss_ctrl_s0_boot_size_proxy(&mut self) -> FssCtrlS0BootSizeProxyW<Cfg0FssCtrlProxySpec> {
        FssCtrlS0BootSizeProxyW::new(self, 8)
    }
}
#[doc = "CFG0_FSS_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_fss_ctrl_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_fss_ctrl_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0FssCtrlProxySpec;
impl crate::RegisterSpec for Cfg0FssCtrlProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_fss_ctrl_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0FssCtrlProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_fss_ctrl_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0FssCtrlProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_FSS_CTRL_PROXY to value 0"]
impl crate::Resettable for Cfg0FssCtrlProxySpec {
    const RESET_VALUE: u32 = 0;
}
