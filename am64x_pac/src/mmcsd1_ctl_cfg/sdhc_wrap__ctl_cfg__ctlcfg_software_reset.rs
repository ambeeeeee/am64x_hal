#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_software_reset` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgSoftwareResetSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_software_reset` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgSoftwareResetSpec>;
#[doc = "Field `SWRST_FOR_ALL` reader - 0:0\\]
This reset affects the entire HC except for the card detection circuit. Register bits of type ROC, RW, RW1C, RWAC are cleared to 0. During its initialization, the HD shall set this bit to 1 to reset the HC. The HC shall reset this bit to 0 when capabilities registers are valid and the HD can read them. Additional use of Software Reset For All may not affect the value of the Capabilities registers. If this bit is set to 1, the SD card shall reset itself and must be re initialized by the HD."]
pub type SwrstForAllR = crate::BitReader;
#[doc = "Field `SWRST_FOR_ALL` writer - 0:0\\]
This reset affects the entire HC except for the card detection circuit. Register bits of type ROC, RW, RW1C, RWAC are cleared to 0. During its initialization, the HD shall set this bit to 1 to reset the HC. The HC shall reset this bit to 0 when capabilities registers are valid and the HD can read them. Additional use of Software Reset For All may not affect the value of the Capabilities registers. If this bit is set to 1, the SD card shall reset itself and must be re initialized by the HD."]
pub type SwrstForAllW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWRST_FOR_CMD` reader - 1:1\\]
Software Reset For CMD Line Only part of command circuit is reset to be able to issue a command. From Version 4.10, this bit is also used to initialize UHS-II command circuit. This reset is effective only command issuing circuit \\[including response error statuses related to Com-mand Inhibit \\[CMD\\]
control\\]
and does not affect data transfer circuit. Host Controller can continue data transfer even this reset is executed during handling of sub com- mand response errors. The following registers and bits are cleared by this bit: Present State register Command Inhibit \\[CMD\\]
Normal Interrupt Status register Command Complete Error Interrupt Status \\[from Version 4.10\\]
Response error statuses related to Com-mand Inhibit \\[CMD\\]"]
pub type SwrstForCmdR = crate::BitReader;
#[doc = "Field `SWRST_FOR_CMD` writer - 1:1\\]
Software Reset For CMD Line Only part of command circuit is reset to be able to issue a command. From Version 4.10, this bit is also used to initialize UHS-II command circuit. This reset is effective only command issuing circuit \\[including response error statuses related to Com-mand Inhibit \\[CMD\\]
control\\]
and does not affect data transfer circuit. Host Controller can continue data transfer even this reset is executed during handling of sub com- mand response errors. The following registers and bits are cleared by this bit: Present State register Command Inhibit \\[CMD\\]
Normal Interrupt Status register Command Complete Error Interrupt Status \\[from Version 4.10\\]
Response error statuses related to Com-mand Inhibit \\[CMD\\]"]
pub type SwrstForCmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWRST_FOR_DAT` reader - 2:2\\]
Only part of data circuit is reset. The following registers and bits are cleared by this bit: Buffer Data Port Register: Buffer is cleared and Initialized. Present State register: Buffer read Enable, Buffer write Enable, Read Transfer Active, Write Transfer Active, DAT Line Active, Command Inhibit \\[DAT\\]. Block Gap Control register: Continue Request, Stop At Block Gap Request. Normal Interrupt Status register: Buffer Read Ready, Buffer Write Ready, Block Gap Event, Transfer Complete."]
pub type SwrstForDatR = crate::BitReader;
#[doc = "Field `SWRST_FOR_DAT` writer - 2:2\\]
Only part of data circuit is reset. The following registers and bits are cleared by this bit: Buffer Data Port Register: Buffer is cleared and Initialized. Present State register: Buffer read Enable, Buffer write Enable, Read Transfer Active, Write Transfer Active, DAT Line Active, Command Inhibit \\[DAT\\]. Block Gap Control register: Continue Request, Stop At Block Gap Request. Normal Interrupt Status register: Buffer Read Ready, Buffer Write Ready, Block Gap Event, Transfer Complete."]
pub type SwrstForDatW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
This reset affects the entire HC except for the card detection circuit. Register bits of type ROC, RW, RW1C, RWAC are cleared to 0. During its initialization, the HD shall set this bit to 1 to reset the HC. The HC shall reset this bit to 0 when capabilities registers are valid and the HD can read them. Additional use of Software Reset For All may not affect the value of the Capabilities registers. If this bit is set to 1, the SD card shall reset itself and must be re initialized by the HD."]
    #[inline(always)]
    pub fn swrst_for_all(&self) -> SwrstForAllR {
        SwrstForAllR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Software Reset For CMD Line Only part of command circuit is reset to be able to issue a command. From Version 4.10, this bit is also used to initialize UHS-II command circuit. This reset is effective only command issuing circuit \\[including response error statuses related to Com-mand Inhibit \\[CMD\\]
control\\]
and does not affect data transfer circuit. Host Controller can continue data transfer even this reset is executed during handling of sub com- mand response errors. The following registers and bits are cleared by this bit: Present State register Command Inhibit \\[CMD\\]
Normal Interrupt Status register Command Complete Error Interrupt Status \\[from Version 4.10\\]
Response error statuses related to Com-mand Inhibit \\[CMD\\]"]
    #[inline(always)]
    pub fn swrst_for_cmd(&self) -> SwrstForCmdR {
        SwrstForCmdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Only part of data circuit is reset. The following registers and bits are cleared by this bit: Buffer Data Port Register: Buffer is cleared and Initialized. Present State register: Buffer read Enable, Buffer write Enable, Read Transfer Active, Write Transfer Active, DAT Line Active, Command Inhibit \\[DAT\\]. Block Gap Control register: Continue Request, Stop At Block Gap Request. Normal Interrupt Status register: Buffer Read Ready, Buffer Write Ready, Block Gap Event, Transfer Complete."]
    #[inline(always)]
    pub fn swrst_for_dat(&self) -> SwrstForDatR {
        SwrstForDatR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
This reset affects the entire HC except for the card detection circuit. Register bits of type ROC, RW, RW1C, RWAC are cleared to 0. During its initialization, the HD shall set this bit to 1 to reset the HC. The HC shall reset this bit to 0 when capabilities registers are valid and the HD can read them. Additional use of Software Reset For All may not affect the value of the Capabilities registers. If this bit is set to 1, the SD card shall reset itself and must be re initialized by the HD."]
    #[inline(always)]
    #[must_use]
    pub fn swrst_for_all(&mut self) -> SwrstForAllW<SdhcWrap_CtlCfg_CtlcfgSoftwareResetSpec> {
        SwrstForAllW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Software Reset For CMD Line Only part of command circuit is reset to be able to issue a command. From Version 4.10, this bit is also used to initialize UHS-II command circuit. This reset is effective only command issuing circuit \\[including response error statuses related to Com-mand Inhibit \\[CMD\\]
control\\]
and does not affect data transfer circuit. Host Controller can continue data transfer even this reset is executed during handling of sub com- mand response errors. The following registers and bits are cleared by this bit: Present State register Command Inhibit \\[CMD\\]
Normal Interrupt Status register Command Complete Error Interrupt Status \\[from Version 4.10\\]
Response error statuses related to Com-mand Inhibit \\[CMD\\]"]
    #[inline(always)]
    #[must_use]
    pub fn swrst_for_cmd(&mut self) -> SwrstForCmdW<SdhcWrap_CtlCfg_CtlcfgSoftwareResetSpec> {
        SwrstForCmdW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Only part of data circuit is reset. The following registers and bits are cleared by this bit: Buffer Data Port Register: Buffer is cleared and Initialized. Present State register: Buffer read Enable, Buffer write Enable, Read Transfer Active, Write Transfer Active, DAT Line Active, Command Inhibit \\[DAT\\]. Block Gap Control register: Continue Request, Stop At Block Gap Request. Normal Interrupt Status register: Buffer Read Ready, Buffer Write Ready, Block Gap Event, Transfer Complete."]
    #[inline(always)]
    #[must_use]
    pub fn swrst_for_dat(&mut self) -> SwrstForDatW<SdhcWrap_CtlCfg_CtlcfgSoftwareResetSpec> {
        SwrstForDatW::new(self, 2)
    }
}
#[doc = "This register is used to program the software reset for data, command and for all\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_software_reset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_software_reset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgSoftwareResetSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgSoftwareResetSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_software_reset::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgSoftwareResetSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_software_reset::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgSoftwareResetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_software_reset to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgSoftwareResetSpec {
    const RESET_VALUE: u8 = 0;
}
