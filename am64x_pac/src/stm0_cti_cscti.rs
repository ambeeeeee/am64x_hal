#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cti__cfg__cscti_cfg_cticontrol: Cti_Cfg_CsctiCfgCticontrol,
    _reserved1: [u8; 0x0c],
    cti__cfg__cscti_cfg_ctiintack: Cti_Cfg_CsctiCfgCtiintack,
    cti__cfg__cscti_cfg_ctiappset: Cti_Cfg_CsctiCfgCtiappset,
    cti__cfg__cscti_cfg_ctiappclear: Cti_Cfg_CsctiCfgCtiappclear,
    cti__cfg__cscti_cfg_ctiapppulse: Cti_Cfg_CsctiCfgCtiapppulse,
    cti__cfg__cscti_cfg_ctiinen0: Cti_Cfg_CsctiCfgCtiinen0,
    cti__cfg__cscti_cfg_ctiinen1: Cti_Cfg_CsctiCfgCtiinen1,
    cti__cfg__cscti_cfg_ctiinen2: Cti_Cfg_CsctiCfgCtiinen2,
    cti__cfg__cscti_cfg_ctiinen3: Cti_Cfg_CsctiCfgCtiinen3,
    cti__cfg__cscti_cfg_ctiinen4: Cti_Cfg_CsctiCfgCtiinen4,
    cti__cfg__cscti_cfg_ctiinen5: Cti_Cfg_CsctiCfgCtiinen5,
    cti__cfg__cscti_cfg_ctiinen6: Cti_Cfg_CsctiCfgCtiinen6,
    cti__cfg__cscti_cfg_ctiinen7: Cti_Cfg_CsctiCfgCtiinen7,
    _reserved13: [u8; 0x60],
    cti__cfg__cscti_cfg_ctiouten0: Cti_Cfg_CsctiCfgCtiouten0,
    cti__cfg__cscti_cfg_ctiouten1: Cti_Cfg_CsctiCfgCtiouten1,
    cti__cfg__cscti_cfg_ctiouten2: Cti_Cfg_CsctiCfgCtiouten2,
    cti__cfg__cscti_cfg_ctiouten3: Cti_Cfg_CsctiCfgCtiouten3,
    cti__cfg__cscti_cfg_ctiouten4: Cti_Cfg_CsctiCfgCtiouten4,
    cti__cfg__cscti_cfg_ctiouten5: Cti_Cfg_CsctiCfgCtiouten5,
    cti__cfg__cscti_cfg_ctiouten6: Cti_Cfg_CsctiCfgCtiouten6,
    cti__cfg__cscti_cfg_ctiouten7: Cti_Cfg_CsctiCfgCtiouten7,
    _reserved21: [u8; 0x70],
    cti__cfg__cscti_cfg_ctitriginstatus: Cti_Cfg_CsctiCfgCtitriginstatus,
    cti__cfg__cscti_cfg_ctitrigoutstatus: Cti_Cfg_CsctiCfgCtitrigoutstatus,
    cti__cfg__cscti_cfg_ctichinstatus: Cti_Cfg_CsctiCfgCtichinstatus,
    cti__cfg__cscti_cfg_ctichoutstatus: Cti_Cfg_CsctiCfgCtichoutstatus,
    cti__cfg__cscti_cfg_ctigate: Cti_Cfg_CsctiCfgCtigate,
    cti__cfg__cscti_cfg_asicctl: Cti_Cfg_CsctiCfgAsicctl,
    _reserved27: [u8; 0x0d94],
    cti__cfg__cscti_cfg_itchinack: Cti_Cfg_CsctiCfgItchinack,
    cti__cfg__cscti_cfg_ittriginack: Cti_Cfg_CsctiCfgIttriginack,
    cti__cfg__cscti_cfg_itchout: Cti_Cfg_CsctiCfgItchout,
    cti__cfg__cscti_cfg_ittrigout: Cti_Cfg_CsctiCfgIttrigout,
    cti__cfg__cscti_cfg_itchoutack: Cti_Cfg_CsctiCfgItchoutack,
    cti__cfg__cscti_cfg_ittrigoutack: Cti_Cfg_CsctiCfgIttrigoutack,
    cti__cfg__cscti_cfg_itchin: Cti_Cfg_CsctiCfgItchin,
    cti__cfg__cscti_cfg_ittrigin: Cti_Cfg_CsctiCfgIttrigin,
    _reserved35: [u8; 0x04],
    cti__cfg__cscti_cfg_itctrl: Cti_Cfg_CsctiCfgItctrl,
    _reserved36: [u8; 0x9c],
    cti__cfg__cscti_cfg_claimset: Cti_Cfg_CsctiCfgClaimset,
    cti__cfg__cscti_cfg_claimclr: Cti_Cfg_CsctiCfgClaimclr,
    _reserved38: [u8; 0x08],
    cti__cfg__cscti_cfg_lar: Cti_Cfg_CsctiCfgLar,
    cti__cfg__cscti_cfg_lsr: Cti_Cfg_CsctiCfgLsr,
    cti__cfg__cscti_cfg_authstatus: Cti_Cfg_CsctiCfgAuthstatus,
    _reserved41: [u8; 0x0c],
    cti__cfg__cscti_cfg_devid: Cti_Cfg_CsctiCfgDevid,
    cti__cfg__cscti_cfg_devtype: Cti_Cfg_CsctiCfgDevtype,
    cti__cfg__cscti_cfg_periphid4: Cti_Cfg_CsctiCfgPeriphid4,
    _reserved44: [u8; 0x0c],
    cti__cfg__cscti_cfg_periphid0: Cti_Cfg_CsctiCfgPeriphid0,
    cti__cfg__cscti_cfg_periphid1: Cti_Cfg_CsctiCfgPeriphid1,
    cti__cfg__cscti_cfg_periphid2: Cti_Cfg_CsctiCfgPeriphid2,
    cti__cfg__cscti_cfg_periphid3: Cti_Cfg_CsctiCfgPeriphid3,
    cti__cfg__cscti_cfg_compid0: Cti_Cfg_CsctiCfgCompid0,
    cti__cfg__cscti_cfg_compid1: Cti_Cfg_CsctiCfgCompid1,
    cti__cfg__cscti_cfg_compid2: Cti_Cfg_CsctiCfgCompid2,
    cti__cfg__cscti_cfg_compid3: Cti_Cfg_CsctiCfgCompid3,
}
impl RegisterBlock {
    #[doc = "0x00 - The CTI Control Register enables the CTI. >"]
    #[inline(always)]
    pub const fn cti__cfg__cscti_cfg_cticontrol(&self) -> &Cti_Cfg_CsctiCfgCticontrol {
        &self.cti__cfg__cscti_cfg_cticontrol
    }
    #[doc = "0x10 - The CTI Interrupt Acknowledge Register is write-only. Any bits written as a 1 cause the ctitrigout output signal to be acknowledged. The acknowledgement is cleared when MAPTRIGOUT is deactivated. This register is used when the ctitrigout is used as a sticky output, that is, no hardware acknowledge is supplied, and a software acknowledge is required."]
    #[inline(always)]
    pub const fn cti__cfg__cscti_cfg_ctiintack(&self) -> &Cti_Cfg_CsctiCfgCtiintack {
        &self.cti__cfg__cscti_cfg_ctiintack
    }
    #[doc = "0x14 - The CTI Application Trigger Set Register is read/write. A write to this register causes a channel event to be raised, corresponding to the bit written to."]
    #[inline(always)]
    pub const fn cti__cfg__cscti_cfg_ctiappset(&self) -> &Cti_Cfg_CsctiCfgCtiappset {
        &self.cti__cfg__cscti_cfg_ctiappset
    }
    #[doc = "0x18 - The CTI Interrupt Acknowledge Register is write-only. A write to this register causes a channel event to be cleared, corresponding to the bit written to."]
    #[inline(always)]
    pub const fn cti__cfg__cscti_cfg_ctiappclear(&self) -> &Cti_Cfg_CsctiCfgCtiappclear {
        &self.cti__cfg__cscti_cfg_ctiappclear
    }
    #[doc = "0x1c - The CTI Application Pulse Register is write-only. A write to this register causes a channel event pulse, one cticlk period, to be generated, corresponding to the bit written to. The pulse external to the ECT can be extended to multi-cycle by the handshaking interface circuits. This register clears itself immediately, so it can be repeatedly written to without software having to clear it."]
    #[inline(always)]
    pub const fn cti__cfg__cscti_cfg_ctiapppulse(&self) -> &Cti_Cfg_CsctiCfgCtiapppulse {
        &self.cti__cfg__cscti_cfg_ctiapppulse
    }
    #[doc = "0x20 - The CTI Trigger 0 to Channel Enable Register enables the signalling of an event on CTM channels when the core issues a trigger, ctitrigin, to the CTI. Within this register there is one bit for each of the four channels implemented. This register does not affect the application trigger operations."]
    #[inline(always)]
    pub const fn cti__cfg__cscti_cfg_ctiinen0(&self) -> &Cti_Cfg_CsctiCfgCtiinen0 {
        &self.cti__cfg__cscti_cfg_ctiinen0
    }
    #[doc = "0x24 - The CTI Trigger 1 to Channel Enable Register enables the signalling of an event on CTM channels when the core issues a trigger, ctitrigin, to the CTI. Within this register there is one bit for each of the four channels implemented. This register does not affect the application trigger operations."]
    #[inline(always)]
    pub const fn cti__cfg__cscti_cfg_ctiinen1(&self) -> &Cti_Cfg_CsctiCfgCtiinen1 {
        &self.cti__cfg__cscti_cfg_ctiinen1
    }
    #[doc = "0x28 - The CTI Trigger to Channel Enable Register 0 enables the signalling of an event on CTM channels when the core issues a trigger, ctitrigin, to the CTI. Within this register there is one bit for each of the four channels implemented. This register does not affect the application trigger operations."]
    #[inline(always)]
    pub const fn cti__cfg__cscti_cfg_ctiinen2(&self) -> &Cti_Cfg_CsctiCfgCtiinen2 {
        &self.cti__cfg__cscti_cfg_ctiinen2
    }
    #[doc = "0x2c - The CTI Trigger to Channel Enable Register 0 enables the signalling of an event on CTM channels when the core issues a trigger, ctitrigin, to the CTI. Within this register there is one bit for each of the four channels implemented. This register does not affect the application trigger operations."]
    #[inline(always)]
    pub const fn cti__cfg__cscti_cfg_ctiinen3(&self) -> &Cti_Cfg_CsctiCfgCtiinen3 {
        &self.cti__cfg__cscti_cfg_ctiinen3
    }
    #[doc = "0x30 - The CTI Trigger to Channel Enable Register 0 enables the signalling of an event on CTM channels when the core issues a trigger, ctitrigin, to the CTI. Within this register there is one bit for each of the four channels implemented. This register does not affect the application trigger operations."]
    #[inline(always)]
    pub const fn cti__cfg__cscti_cfg_ctiinen4(&self) -> &Cti_Cfg_CsctiCfgCtiinen4 {
        &self.cti__cfg__cscti_cfg_ctiinen4
    }
    #[doc = "0x34 - The CTI Trigger to Channel Enable Register 0 enables the signalling of an event on CTM channels when the core issues a trigger, ctitrigin, to the CTI. Within this register there is one bit for each of the four channels implemented. This register does not affect the application trigger operations."]
    #[inline(always)]
    pub const fn cti__cfg__cscti_cfg_ctiinen5(&self) -> &Cti_Cfg_CsctiCfgCtiinen5 {
        &self.cti__cfg__cscti_cfg_ctiinen5
    }
    #[doc = "0x38 - The CTI Trigger to Channel Enable Register 0 enables the signalling of an event on CTM channels when the core issues a trigger, ctitrigin, to the CTI. Within this register there is one bit for each of the four channels implemented. This register does not affect the application trigger operations."]
    #[inline(always)]
    pub const fn cti__cfg__cscti_cfg_ctiinen6(&self) -> &Cti_Cfg_CsctiCfgCtiinen6 {
        &self.cti__cfg__cscti_cfg_ctiinen6
    }
    #[doc = "0x3c - The CTI Trigger to Channel Enable Register 0 enables the signalling of an event on CTM channels when the core issues a trigger, ctitrigin, to the CTI. Within this register there is one bit for each of the four channels implemented. This register does not affect the application trigger operations."]
    #[inline(always)]
    pub const fn cti__cfg__cscti_cfg_ctiinen7(&self) -> &Cti_Cfg_CsctiCfgCtiinen7 {
        &self.cti__cfg__cscti_cfg_ctiinen7
    }
    #[doc = "0xa0 - The CTI Channel to Trigger 0 Enable Registers define which channels can generate a ctitrigout\\[0\\]
output. Within this register there is one bit for each of the four channels implemented. This register affects the mapping from application trigger to trigger outputs."]
    #[inline(always)]
    pub const fn cti__cfg__cscti_cfg_ctiouten0(&self) -> &Cti_Cfg_CsctiCfgCtiouten0 {
        &self.cti__cfg__cscti_cfg_ctiouten0
    }
    #[doc = "0xa4 - The CTI Channel to Trigger 1 Enable Registers define which channels can generate a ctitrigout\\[1\\]
output. Within this register there is one bit for each of the four channels implemented. This register affects the mapping from application trigger to trigger outputs."]
    #[inline(always)]
    pub const fn cti__cfg__cscti_cfg_ctiouten1(&self) -> &Cti_Cfg_CsctiCfgCtiouten1 {
        &self.cti__cfg__cscti_cfg_ctiouten1
    }
    #[doc = "0xa8 - The CTI Channel to Trigger 2 Enable Registers define which channels can generate a ctitrigout\\[2\\]
output. Within this register there is one bit for each of the four channels implemented. This register affects the mapping from application trigger to trigger outputs."]
    #[inline(always)]
    pub const fn cti__cfg__cscti_cfg_ctiouten2(&self) -> &Cti_Cfg_CsctiCfgCtiouten2 {
        &self.cti__cfg__cscti_cfg_ctiouten2
    }
    #[doc = "0xac - The CTI Channel to Trigger 3 Enable Registers define which channels can generate a ctitrigout\\[3\\]
output. Within this register there is one bit for each of the four channels implemented. This register affects the mapping from application trigger to trigger outputs."]
    #[inline(always)]
    pub const fn cti__cfg__cscti_cfg_ctiouten3(&self) -> &Cti_Cfg_CsctiCfgCtiouten3 {
        &self.cti__cfg__cscti_cfg_ctiouten3
    }
    #[doc = "0xb0 - The CTI Channel to Trigger 4 Enable Registers define which channels can generate a ctitrigout\\[4\\]
output. Within this register there is one bit for each of the four channels implemented. This register affects the mapping from application trigger to trigger outputs."]
    #[inline(always)]
    pub const fn cti__cfg__cscti_cfg_ctiouten4(&self) -> &Cti_Cfg_CsctiCfgCtiouten4 {
        &self.cti__cfg__cscti_cfg_ctiouten4
    }
    #[doc = "0xb4 - The CTI Channel to Trigger 5 Enable Registers define which channels can generate a ctitrigout\\[5\\]
output. Within this register there is one bit for each of the four channels implemented. This register affects the mapping from application trigger to trigger outputs."]
    #[inline(always)]
    pub const fn cti__cfg__cscti_cfg_ctiouten5(&self) -> &Cti_Cfg_CsctiCfgCtiouten5 {
        &self.cti__cfg__cscti_cfg_ctiouten5
    }
    #[doc = "0xb8 - The CTI Channel to Trigger 6 Enable Registers define which channels can generate a ctitrigout\\[6\\]
output. Within this register there is one bit for each of the four channels implemented. This register affects the mapping from application trigger to trigger outputs."]
    #[inline(always)]
    pub const fn cti__cfg__cscti_cfg_ctiouten6(&self) -> &Cti_Cfg_CsctiCfgCtiouten6 {
        &self.cti__cfg__cscti_cfg_ctiouten6
    }
    #[doc = "0xbc - The CTI Channel to Trigger 7 Enable Registers define which channels can generate a ctitrigout\\[7\\]
output. Within this register there is one bit for each of the four channels implemented. This register affects the mapping from application trigger to trigger outputs."]
    #[inline(always)]
    pub const fn cti__cfg__cscti_cfg_ctiouten7(&self) -> &Cti_Cfg_CsctiCfgCtiouten7 {
        &self.cti__cfg__cscti_cfg_ctiouten7
    }
    #[doc = "0x130 - The CTI Trigger In Status Register provides the status of the ctitrigin inputs."]
    #[inline(always)]
    pub const fn cti__cfg__cscti_cfg_ctitriginstatus(&self) -> &Cti_Cfg_CsctiCfgCtitriginstatus {
        &self.cti__cfg__cscti_cfg_ctitriginstatus
    }
    #[doc = "0x134 - The CTI Trigger Out Status Register provides the status of the ctitrigout outputs."]
    #[inline(always)]
    pub const fn cti__cfg__cscti_cfg_ctitrigoutstatus(&self) -> &Cti_Cfg_CsctiCfgCtitrigoutstatus {
        &self.cti__cfg__cscti_cfg_ctitrigoutstatus
    }
    #[doc = "0x138 - The CTI Channel In Status Register provides the status of the ctichin inputs."]
    #[inline(always)]
    pub const fn cti__cfg__cscti_cfg_ctichinstatus(&self) -> &Cti_Cfg_CsctiCfgCtichinstatus {
        &self.cti__cfg__cscti_cfg_ctichinstatus
    }
    #[doc = "0x13c - The CTI Channel Out Status Register provides the status of the CTI ctichout outputs."]
    #[inline(always)]
    pub const fn cti__cfg__cscti_cfg_ctichoutstatus(&self) -> &Cti_Cfg_CsctiCfgCtichoutstatus {
        &self.cti__cfg__cscti_cfg_ctichoutstatus
    }
    #[doc = "0x140 - The Gate Enable Register prevents the channels from propagating through the CTM to other CTIs. This enables local cross-triggering, for example for causing an interrupt when the ETM trigger occurs. It can be used effectively with CTIAPPSET, CTIAPPCLEAR, and CTIAPPPULSE for asserting trigger outputs by asserting channels, without affecting the rest of the system. On reset, this register is 0xF, and channel propagation is enabled."]
    #[inline(always)]
    pub const fn cti__cfg__cscti_cfg_ctigate(&self) -> &Cti_Cfg_CsctiCfgCtigate {
        &self.cti__cfg__cscti_cfg_ctigate
    }
    #[doc = "0x144 - Implementation-defined ASIC control, value written to the register is output on asicctl\\[7 : 0\\]."]
    #[inline(always)]
    pub const fn cti__cfg__cscti_cfg_asicctl(&self) -> &Cti_Cfg_CsctiCfgAsicctl {
        &self.cti__cfg__cscti_cfg_asicctl
    }
    #[doc = "0xedc - This register is a write-only register. It can be used to set the value of the CTCHINACK outputs."]
    #[inline(always)]
    pub const fn cti__cfg__cscti_cfg_itchinack(&self) -> &Cti_Cfg_CsctiCfgItchinack {
        &self.cti__cfg__cscti_cfg_itchinack
    }
    #[doc = "0xee0 - This register is a write-only register. It can be used to set the value of the CTTRIGINACK outputs."]
    #[inline(always)]
    pub const fn cti__cfg__cscti_cfg_ittriginack(&self) -> &Cti_Cfg_CsctiCfgIttriginack {
        &self.cti__cfg__cscti_cfg_ittriginack
    }
    #[doc = "0xee4 - This register is a write-only register. It can be used to set the value of the CTCHOUT outputs."]
    #[inline(always)]
    pub const fn cti__cfg__cscti_cfg_itchout(&self) -> &Cti_Cfg_CsctiCfgItchout {
        &self.cti__cfg__cscti_cfg_itchout
    }
    #[doc = "0xee8 - This register is a write-only register. It can be used to set the value of the CTTRIGOUT outputs."]
    #[inline(always)]
    pub const fn cti__cfg__cscti_cfg_ittrigout(&self) -> &Cti_Cfg_CsctiCfgIttrigout {
        &self.cti__cfg__cscti_cfg_ittrigout
    }
    #[doc = "0xeec - This register is a read-only register. It can be used to read the values of the CTCHOUTACK inputs."]
    #[inline(always)]
    pub const fn cti__cfg__cscti_cfg_itchoutack(&self) -> &Cti_Cfg_CsctiCfgItchoutack {
        &self.cti__cfg__cscti_cfg_itchoutack
    }
    #[doc = "0xef0 - This register is a read-only register. It can be used to read the values of the CTTRIGOUTACK inputs."]
    #[inline(always)]
    pub const fn cti__cfg__cscti_cfg_ittrigoutack(&self) -> &Cti_Cfg_CsctiCfgIttrigoutack {
        &self.cti__cfg__cscti_cfg_ittrigoutack
    }
    #[doc = "0xef4 - This register is a read-only register. It can be used to read the values of the CTCHIN inputs."]
    #[inline(always)]
    pub const fn cti__cfg__cscti_cfg_itchin(&self) -> &Cti_Cfg_CsctiCfgItchin {
        &self.cti__cfg__cscti_cfg_itchin
    }
    #[doc = "0xef8 - This register is a read-only register. It can be used to read the values of the CTTRIGIN inputs."]
    #[inline(always)]
    pub const fn cti__cfg__cscti_cfg_ittrigin(&self) -> &Cti_Cfg_CsctiCfgIttrigin {
        &self.cti__cfg__cscti_cfg_ittrigin
    }
    #[doc = "0xf00 - This register is used to enable topology detection. For more information see the CoreSight Architecture Specification. This register enables the component to switch from a functional mode, the default behavior, to integration mode where the inputs and outputs of the component can be directly controlled for the purpose of integration testing and topology solving. Note : When a device has been in integration mode, it might not function with the original behavior. After performing integration or topology detection, you must reset the system to ensure correct behavior of CoreSight and other connected system components that are affected by the integration or topology detection."]
    #[inline(always)]
    pub const fn cti__cfg__cscti_cfg_itctrl(&self) -> &Cti_Cfg_CsctiCfgItctrl {
        &self.cti__cfg__cscti_cfg_itctrl
    }
    #[doc = "0xfa0 - This is used in conjunction with Claim Tag Clear Register, CLAIMCLR. This register forms one half of the Claim Tag value. This location allows individual bits to be set, write, and returns the number of bits that can be set, read."]
    #[inline(always)]
    pub const fn cti__cfg__cscti_cfg_claimset(&self) -> &Cti_Cfg_CsctiCfgClaimset {
        &self.cti__cfg__cscti_cfg_claimset
    }
    #[doc = "0xfa4 - This register is used in conjunction with Claim Tag Set Register, CLAIMSET. This register forms one half of the Claim Tag value. This location enables individual bits to be cleared, write, and returns the current Claim Tag value, read."]
    #[inline(always)]
    pub const fn cti__cfg__cscti_cfg_claimclr(&self) -> &Cti_Cfg_CsctiCfgClaimclr {
        &self.cti__cfg__cscti_cfg_claimclr
    }
    #[doc = "0xfb0 - This is used to enable write access to device registers. External accesses from a debugger (paddrdbg31 = 1) are not subject to the Lock Registers. A debugger does not have to unlock the component in order to write and modify the registers in the component."]
    #[inline(always)]
    pub const fn cti__cfg__cscti_cfg_lar(&self) -> &Cti_Cfg_CsctiCfgLar {
        &self.cti__cfg__cscti_cfg_lar
    }
    #[doc = "0xfb4 - This indicates the status of the Lock control mechanism. This lock prevents accidental writes by code under debug. When locked, write access is blocked to all registers, except the Lock Access Register. External accesses from a debugger (paddrdbg31 = 1) are not subject to the Lock Registers. This register reads as 0 when read from an external debugger (paddrdbg31 = 1)."]
    #[inline(always)]
    pub const fn cti__cfg__cscti_cfg_lsr(&self) -> &Cti_Cfg_CsctiCfgLsr {
        &self.cti__cfg__cscti_cfg_lsr
    }
    #[doc = "0xfb8 - Reports what functionality is currently permitted by the authentication interface."]
    #[inline(always)]
    pub const fn cti__cfg__cscti_cfg_authstatus(&self) -> &Cti_Cfg_CsctiCfgAuthstatus {
        &self.cti__cfg__cscti_cfg_authstatus
    }
    #[doc = "0xfc8 - This register indicates the capabilities of the CTI."]
    #[inline(always)]
    pub const fn cti__cfg__cscti_cfg_devid(&self) -> &Cti_Cfg_CsctiCfgDevid {
        &self.cti__cfg__cscti_cfg_devid
    }
    #[doc = "0xfcc - It provides a debugger with information about the component when the Part Number field is not recognized. The debugger can then report this information."]
    #[inline(always)]
    pub const fn cti__cfg__cscti_cfg_devtype(&self) -> &Cti_Cfg_CsctiCfgDevtype {
        &self.cti__cfg__cscti_cfg_devtype
    }
    #[doc = "0xfd0 - Part of the set of Peripheral Identification registers. Contains part of the designer identity and the memory footprint indicator."]
    #[inline(always)]
    pub const fn cti__cfg__cscti_cfg_periphid4(&self) -> &Cti_Cfg_CsctiCfgPeriphid4 {
        &self.cti__cfg__cscti_cfg_periphid4
    }
    #[doc = "0xfe0 - Part of the set of Peripheral Identification registers. Contains part of the designer specific part number."]
    #[inline(always)]
    pub const fn cti__cfg__cscti_cfg_periphid0(&self) -> &Cti_Cfg_CsctiCfgPeriphid0 {
        &self.cti__cfg__cscti_cfg_periphid0
    }
    #[doc = "0xfe4 - Part of the set of Peripheral Identification registers. Contains part of the designer specific part number and part of the designer identity."]
    #[inline(always)]
    pub const fn cti__cfg__cscti_cfg_periphid1(&self) -> &Cti_Cfg_CsctiCfgPeriphid1 {
        &self.cti__cfg__cscti_cfg_periphid1
    }
    #[doc = "0xfe8 - Part of the set of Peripheral Identification registers. Contains part of the designer identity and the product revision."]
    #[inline(always)]
    pub const fn cti__cfg__cscti_cfg_periphid2(&self) -> &Cti_Cfg_CsctiCfgPeriphid2 {
        &self.cti__cfg__cscti_cfg_periphid2
    }
    #[doc = "0xfec - Part of the set of Peripheral Identification registers. Contains the RevAnd and Customer Modified fields."]
    #[inline(always)]
    pub const fn cti__cfg__cscti_cfg_periphid3(&self) -> &Cti_Cfg_CsctiCfgPeriphid3 {
        &self.cti__cfg__cscti_cfg_periphid3
    }
    #[doc = "0xff0 - Reserved Reserved Reserved A component identification register, that indicates that the identification registers are present."]
    #[inline(always)]
    pub const fn cti__cfg__cscti_cfg_compid0(&self) -> &Cti_Cfg_CsctiCfgCompid0 {
        &self.cti__cfg__cscti_cfg_compid0
    }
    #[doc = "0xff4 - A component identification register, that indicates that the identification registers are present. This register also indicates the component class."]
    #[inline(always)]
    pub const fn cti__cfg__cscti_cfg_compid1(&self) -> &Cti_Cfg_CsctiCfgCompid1 {
        &self.cti__cfg__cscti_cfg_compid1
    }
    #[doc = "0xff8 - A component identification register, that indicates that the identification registers are present."]
    #[inline(always)]
    pub const fn cti__cfg__cscti_cfg_compid2(&self) -> &Cti_Cfg_CsctiCfgCompid2 {
        &self.cti__cfg__cscti_cfg_compid2
    }
    #[doc = "0xffc - A component identification register, that indicates that the identification registers are present."]
    #[inline(always)]
    pub const fn cti__cfg__cscti_cfg_compid3(&self) -> &Cti_Cfg_CsctiCfgCompid3 {
        &self.cti__cfg__cscti_cfg_compid3
    }
}
#[doc = "CTI__CFG__CSCTI_CFG_CTICONTROL (rw) register accessor: The CTI Control Register enables the CTI. >\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_cticontrol::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_cticontrol::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cti__cfg__cscti_cfg_cticontrol`]
module"]
#[doc(alias = "CTI__CFG__CSCTI_CFG_CTICONTROL")]
pub type Cti_Cfg_CsctiCfgCticontrol =
    crate::Reg<cti__cfg__cscti_cfg_cticontrol::Cti_Cfg_CsctiCfgCticontrolSpec>;
#[doc = "The CTI Control Register enables the CTI. >"]
pub mod cti__cfg__cscti_cfg_cticontrol;
#[doc = "CTI__CFG__CSCTI_CFG_CTIINTACK (rw) register accessor: The CTI Interrupt Acknowledge Register is write-only. Any bits written as a 1 cause the ctitrigout output signal to be acknowledged. The acknowledgement is cleared when MAPTRIGOUT is deactivated. This register is used when the ctitrigout is used as a sticky output, that is, no hardware acknowledge is supplied, and a software acknowledge is required.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_ctiintack::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_ctiintack::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cti__cfg__cscti_cfg_ctiintack`]
module"]
#[doc(alias = "CTI__CFG__CSCTI_CFG_CTIINTACK")]
pub type Cti_Cfg_CsctiCfgCtiintack =
    crate::Reg<cti__cfg__cscti_cfg_ctiintack::Cti_Cfg_CsctiCfgCtiintackSpec>;
#[doc = "The CTI Interrupt Acknowledge Register is write-only. Any bits written as a 1 cause the ctitrigout output signal to be acknowledged. The acknowledgement is cleared when MAPTRIGOUT is deactivated. This register is used when the ctitrigout is used as a sticky output, that is, no hardware acknowledge is supplied, and a software acknowledge is required."]
pub mod cti__cfg__cscti_cfg_ctiintack;
#[doc = "CTI__CFG__CSCTI_CFG_CTIAPPSET (rw) register accessor: The CTI Application Trigger Set Register is read/write. A write to this register causes a channel event to be raised, corresponding to the bit written to.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_ctiappset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_ctiappset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cti__cfg__cscti_cfg_ctiappset`]
module"]
#[doc(alias = "CTI__CFG__CSCTI_CFG_CTIAPPSET")]
pub type Cti_Cfg_CsctiCfgCtiappset =
    crate::Reg<cti__cfg__cscti_cfg_ctiappset::Cti_Cfg_CsctiCfgCtiappsetSpec>;
#[doc = "The CTI Application Trigger Set Register is read/write. A write to this register causes a channel event to be raised, corresponding to the bit written to."]
pub mod cti__cfg__cscti_cfg_ctiappset;
#[doc = "CTI__CFG__CSCTI_CFG_CTIAPPCLEAR (rw) register accessor: The CTI Interrupt Acknowledge Register is write-only. A write to this register causes a channel event to be cleared, corresponding to the bit written to.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_ctiappclear::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_ctiappclear::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cti__cfg__cscti_cfg_ctiappclear`]
module"]
#[doc(alias = "CTI__CFG__CSCTI_CFG_CTIAPPCLEAR")]
pub type Cti_Cfg_CsctiCfgCtiappclear =
    crate::Reg<cti__cfg__cscti_cfg_ctiappclear::Cti_Cfg_CsctiCfgCtiappclearSpec>;
#[doc = "The CTI Interrupt Acknowledge Register is write-only. A write to this register causes a channel event to be cleared, corresponding to the bit written to."]
pub mod cti__cfg__cscti_cfg_ctiappclear;
#[doc = "CTI__CFG__CSCTI_CFG_CTIAPPPULSE (rw) register accessor: The CTI Application Pulse Register is write-only. A write to this register causes a channel event pulse, one cticlk period, to be generated, corresponding to the bit written to. The pulse external to the ECT can be extended to multi-cycle by the handshaking interface circuits. This register clears itself immediately, so it can be repeatedly written to without software having to clear it.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_ctiapppulse::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_ctiapppulse::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cti__cfg__cscti_cfg_ctiapppulse`]
module"]
#[doc(alias = "CTI__CFG__CSCTI_CFG_CTIAPPPULSE")]
pub type Cti_Cfg_CsctiCfgCtiapppulse =
    crate::Reg<cti__cfg__cscti_cfg_ctiapppulse::Cti_Cfg_CsctiCfgCtiapppulseSpec>;
#[doc = "The CTI Application Pulse Register is write-only. A write to this register causes a channel event pulse, one cticlk period, to be generated, corresponding to the bit written to. The pulse external to the ECT can be extended to multi-cycle by the handshaking interface circuits. This register clears itself immediately, so it can be repeatedly written to without software having to clear it."]
pub mod cti__cfg__cscti_cfg_ctiapppulse;
#[doc = "CTI__CFG__CSCTI_CFG_CTIINEN0 (rw) register accessor: The CTI Trigger 0 to Channel Enable Register enables the signalling of an event on CTM channels when the core issues a trigger, ctitrigin, to the CTI. Within this register there is one bit for each of the four channels implemented. This register does not affect the application trigger operations.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_ctiinen0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_ctiinen0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cti__cfg__cscti_cfg_ctiinen0`]
module"]
#[doc(alias = "CTI__CFG__CSCTI_CFG_CTIINEN0")]
pub type Cti_Cfg_CsctiCfgCtiinen0 =
    crate::Reg<cti__cfg__cscti_cfg_ctiinen0::Cti_Cfg_CsctiCfgCtiinen0Spec>;
#[doc = "The CTI Trigger 0 to Channel Enable Register enables the signalling of an event on CTM channels when the core issues a trigger, ctitrigin, to the CTI. Within this register there is one bit for each of the four channels implemented. This register does not affect the application trigger operations."]
pub mod cti__cfg__cscti_cfg_ctiinen0;
#[doc = "CTI__CFG__CSCTI_CFG_CTIINEN1 (rw) register accessor: The CTI Trigger 1 to Channel Enable Register enables the signalling of an event on CTM channels when the core issues a trigger, ctitrigin, to the CTI. Within this register there is one bit for each of the four channels implemented. This register does not affect the application trigger operations.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_ctiinen1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_ctiinen1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cti__cfg__cscti_cfg_ctiinen1`]
module"]
#[doc(alias = "CTI__CFG__CSCTI_CFG_CTIINEN1")]
pub type Cti_Cfg_CsctiCfgCtiinen1 =
    crate::Reg<cti__cfg__cscti_cfg_ctiinen1::Cti_Cfg_CsctiCfgCtiinen1Spec>;
#[doc = "The CTI Trigger 1 to Channel Enable Register enables the signalling of an event on CTM channels when the core issues a trigger, ctitrigin, to the CTI. Within this register there is one bit for each of the four channels implemented. This register does not affect the application trigger operations."]
pub mod cti__cfg__cscti_cfg_ctiinen1;
#[doc = "CTI__CFG__CSCTI_CFG_CTIINEN2 (rw) register accessor: The CTI Trigger to Channel Enable Register 0 enables the signalling of an event on CTM channels when the core issues a trigger, ctitrigin, to the CTI. Within this register there is one bit for each of the four channels implemented. This register does not affect the application trigger operations.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_ctiinen2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_ctiinen2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cti__cfg__cscti_cfg_ctiinen2`]
module"]
#[doc(alias = "CTI__CFG__CSCTI_CFG_CTIINEN2")]
pub type Cti_Cfg_CsctiCfgCtiinen2 =
    crate::Reg<cti__cfg__cscti_cfg_ctiinen2::Cti_Cfg_CsctiCfgCtiinen2Spec>;
#[doc = "The CTI Trigger to Channel Enable Register 0 enables the signalling of an event on CTM channels when the core issues a trigger, ctitrigin, to the CTI. Within this register there is one bit for each of the four channels implemented. This register does not affect the application trigger operations."]
pub mod cti__cfg__cscti_cfg_ctiinen2;
#[doc = "CTI__CFG__CSCTI_CFG_CTIINEN3 (rw) register accessor: The CTI Trigger to Channel Enable Register 0 enables the signalling of an event on CTM channels when the core issues a trigger, ctitrigin, to the CTI. Within this register there is one bit for each of the four channels implemented. This register does not affect the application trigger operations.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_ctiinen3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_ctiinen3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cti__cfg__cscti_cfg_ctiinen3`]
module"]
#[doc(alias = "CTI__CFG__CSCTI_CFG_CTIINEN3")]
pub type Cti_Cfg_CsctiCfgCtiinen3 =
    crate::Reg<cti__cfg__cscti_cfg_ctiinen3::Cti_Cfg_CsctiCfgCtiinen3Spec>;
#[doc = "The CTI Trigger to Channel Enable Register 0 enables the signalling of an event on CTM channels when the core issues a trigger, ctitrigin, to the CTI. Within this register there is one bit for each of the four channels implemented. This register does not affect the application trigger operations."]
pub mod cti__cfg__cscti_cfg_ctiinen3;
#[doc = "CTI__CFG__CSCTI_CFG_CTIINEN4 (rw) register accessor: The CTI Trigger to Channel Enable Register 0 enables the signalling of an event on CTM channels when the core issues a trigger, ctitrigin, to the CTI. Within this register there is one bit for each of the four channels implemented. This register does not affect the application trigger operations.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_ctiinen4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_ctiinen4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cti__cfg__cscti_cfg_ctiinen4`]
module"]
#[doc(alias = "CTI__CFG__CSCTI_CFG_CTIINEN4")]
pub type Cti_Cfg_CsctiCfgCtiinen4 =
    crate::Reg<cti__cfg__cscti_cfg_ctiinen4::Cti_Cfg_CsctiCfgCtiinen4Spec>;
#[doc = "The CTI Trigger to Channel Enable Register 0 enables the signalling of an event on CTM channels when the core issues a trigger, ctitrigin, to the CTI. Within this register there is one bit for each of the four channels implemented. This register does not affect the application trigger operations."]
pub mod cti__cfg__cscti_cfg_ctiinen4;
#[doc = "CTI__CFG__CSCTI_CFG_CTIINEN5 (rw) register accessor: The CTI Trigger to Channel Enable Register 0 enables the signalling of an event on CTM channels when the core issues a trigger, ctitrigin, to the CTI. Within this register there is one bit for each of the four channels implemented. This register does not affect the application trigger operations.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_ctiinen5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_ctiinen5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cti__cfg__cscti_cfg_ctiinen5`]
module"]
#[doc(alias = "CTI__CFG__CSCTI_CFG_CTIINEN5")]
pub type Cti_Cfg_CsctiCfgCtiinen5 =
    crate::Reg<cti__cfg__cscti_cfg_ctiinen5::Cti_Cfg_CsctiCfgCtiinen5Spec>;
#[doc = "The CTI Trigger to Channel Enable Register 0 enables the signalling of an event on CTM channels when the core issues a trigger, ctitrigin, to the CTI. Within this register there is one bit for each of the four channels implemented. This register does not affect the application trigger operations."]
pub mod cti__cfg__cscti_cfg_ctiinen5;
#[doc = "CTI__CFG__CSCTI_CFG_CTIINEN6 (rw) register accessor: The CTI Trigger to Channel Enable Register 0 enables the signalling of an event on CTM channels when the core issues a trigger, ctitrigin, to the CTI. Within this register there is one bit for each of the four channels implemented. This register does not affect the application trigger operations.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_ctiinen6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_ctiinen6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cti__cfg__cscti_cfg_ctiinen6`]
module"]
#[doc(alias = "CTI__CFG__CSCTI_CFG_CTIINEN6")]
pub type Cti_Cfg_CsctiCfgCtiinen6 =
    crate::Reg<cti__cfg__cscti_cfg_ctiinen6::Cti_Cfg_CsctiCfgCtiinen6Spec>;
#[doc = "The CTI Trigger to Channel Enable Register 0 enables the signalling of an event on CTM channels when the core issues a trigger, ctitrigin, to the CTI. Within this register there is one bit for each of the four channels implemented. This register does not affect the application trigger operations."]
pub mod cti__cfg__cscti_cfg_ctiinen6;
#[doc = "CTI__CFG__CSCTI_CFG_CTIINEN7 (rw) register accessor: The CTI Trigger to Channel Enable Register 0 enables the signalling of an event on CTM channels when the core issues a trigger, ctitrigin, to the CTI. Within this register there is one bit for each of the four channels implemented. This register does not affect the application trigger operations.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_ctiinen7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_ctiinen7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cti__cfg__cscti_cfg_ctiinen7`]
module"]
#[doc(alias = "CTI__CFG__CSCTI_CFG_CTIINEN7")]
pub type Cti_Cfg_CsctiCfgCtiinen7 =
    crate::Reg<cti__cfg__cscti_cfg_ctiinen7::Cti_Cfg_CsctiCfgCtiinen7Spec>;
#[doc = "The CTI Trigger to Channel Enable Register 0 enables the signalling of an event on CTM channels when the core issues a trigger, ctitrigin, to the CTI. Within this register there is one bit for each of the four channels implemented. This register does not affect the application trigger operations."]
pub mod cti__cfg__cscti_cfg_ctiinen7;
#[doc = "CTI__CFG__CSCTI_CFG_CTIOUTEN0 (rw) register accessor: The CTI Channel to Trigger 0 Enable Registers define which channels can generate a ctitrigout\\[0\\]
output. Within this register there is one bit for each of the four channels implemented. This register affects the mapping from application trigger to trigger outputs.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_ctiouten0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_ctiouten0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cti__cfg__cscti_cfg_ctiouten0`]
module"]
#[doc(alias = "CTI__CFG__CSCTI_CFG_CTIOUTEN0")]
pub type Cti_Cfg_CsctiCfgCtiouten0 =
    crate::Reg<cti__cfg__cscti_cfg_ctiouten0::Cti_Cfg_CsctiCfgCtiouten0Spec>;
#[doc = "The CTI Channel to Trigger 0 Enable Registers define which channels can generate a ctitrigout\\[0\\]
output. Within this register there is one bit for each of the four channels implemented. This register affects the mapping from application trigger to trigger outputs."]
pub mod cti__cfg__cscti_cfg_ctiouten0;
#[doc = "CTI__CFG__CSCTI_CFG_CTIOUTEN1 (rw) register accessor: The CTI Channel to Trigger 1 Enable Registers define which channels can generate a ctitrigout\\[1\\]
output. Within this register there is one bit for each of the four channels implemented. This register affects the mapping from application trigger to trigger outputs.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_ctiouten1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_ctiouten1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cti__cfg__cscti_cfg_ctiouten1`]
module"]
#[doc(alias = "CTI__CFG__CSCTI_CFG_CTIOUTEN1")]
pub type Cti_Cfg_CsctiCfgCtiouten1 =
    crate::Reg<cti__cfg__cscti_cfg_ctiouten1::Cti_Cfg_CsctiCfgCtiouten1Spec>;
#[doc = "The CTI Channel to Trigger 1 Enable Registers define which channels can generate a ctitrigout\\[1\\]
output. Within this register there is one bit for each of the four channels implemented. This register affects the mapping from application trigger to trigger outputs."]
pub mod cti__cfg__cscti_cfg_ctiouten1;
#[doc = "CTI__CFG__CSCTI_CFG_CTIOUTEN2 (rw) register accessor: The CTI Channel to Trigger 2 Enable Registers define which channels can generate a ctitrigout\\[2\\]
output. Within this register there is one bit for each of the four channels implemented. This register affects the mapping from application trigger to trigger outputs.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_ctiouten2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_ctiouten2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cti__cfg__cscti_cfg_ctiouten2`]
module"]
#[doc(alias = "CTI__CFG__CSCTI_CFG_CTIOUTEN2")]
pub type Cti_Cfg_CsctiCfgCtiouten2 =
    crate::Reg<cti__cfg__cscti_cfg_ctiouten2::Cti_Cfg_CsctiCfgCtiouten2Spec>;
#[doc = "The CTI Channel to Trigger 2 Enable Registers define which channels can generate a ctitrigout\\[2\\]
output. Within this register there is one bit for each of the four channels implemented. This register affects the mapping from application trigger to trigger outputs."]
pub mod cti__cfg__cscti_cfg_ctiouten2;
#[doc = "CTI__CFG__CSCTI_CFG_CTIOUTEN3 (rw) register accessor: The CTI Channel to Trigger 3 Enable Registers define which channels can generate a ctitrigout\\[3\\]
output. Within this register there is one bit for each of the four channels implemented. This register affects the mapping from application trigger to trigger outputs.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_ctiouten3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_ctiouten3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cti__cfg__cscti_cfg_ctiouten3`]
module"]
#[doc(alias = "CTI__CFG__CSCTI_CFG_CTIOUTEN3")]
pub type Cti_Cfg_CsctiCfgCtiouten3 =
    crate::Reg<cti__cfg__cscti_cfg_ctiouten3::Cti_Cfg_CsctiCfgCtiouten3Spec>;
#[doc = "The CTI Channel to Trigger 3 Enable Registers define which channels can generate a ctitrigout\\[3\\]
output. Within this register there is one bit for each of the four channels implemented. This register affects the mapping from application trigger to trigger outputs."]
pub mod cti__cfg__cscti_cfg_ctiouten3;
#[doc = "CTI__CFG__CSCTI_CFG_CTIOUTEN4 (rw) register accessor: The CTI Channel to Trigger 4 Enable Registers define which channels can generate a ctitrigout\\[4\\]
output. Within this register there is one bit for each of the four channels implemented. This register affects the mapping from application trigger to trigger outputs.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_ctiouten4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_ctiouten4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cti__cfg__cscti_cfg_ctiouten4`]
module"]
#[doc(alias = "CTI__CFG__CSCTI_CFG_CTIOUTEN4")]
pub type Cti_Cfg_CsctiCfgCtiouten4 =
    crate::Reg<cti__cfg__cscti_cfg_ctiouten4::Cti_Cfg_CsctiCfgCtiouten4Spec>;
#[doc = "The CTI Channel to Trigger 4 Enable Registers define which channels can generate a ctitrigout\\[4\\]
output. Within this register there is one bit for each of the four channels implemented. This register affects the mapping from application trigger to trigger outputs."]
pub mod cti__cfg__cscti_cfg_ctiouten4;
#[doc = "CTI__CFG__CSCTI_CFG_CTIOUTEN5 (rw) register accessor: The CTI Channel to Trigger 5 Enable Registers define which channels can generate a ctitrigout\\[5\\]
output. Within this register there is one bit for each of the four channels implemented. This register affects the mapping from application trigger to trigger outputs.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_ctiouten5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_ctiouten5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cti__cfg__cscti_cfg_ctiouten5`]
module"]
#[doc(alias = "CTI__CFG__CSCTI_CFG_CTIOUTEN5")]
pub type Cti_Cfg_CsctiCfgCtiouten5 =
    crate::Reg<cti__cfg__cscti_cfg_ctiouten5::Cti_Cfg_CsctiCfgCtiouten5Spec>;
#[doc = "The CTI Channel to Trigger 5 Enable Registers define which channels can generate a ctitrigout\\[5\\]
output. Within this register there is one bit for each of the four channels implemented. This register affects the mapping from application trigger to trigger outputs."]
pub mod cti__cfg__cscti_cfg_ctiouten5;
#[doc = "CTI__CFG__CSCTI_CFG_CTIOUTEN6 (rw) register accessor: The CTI Channel to Trigger 6 Enable Registers define which channels can generate a ctitrigout\\[6\\]
output. Within this register there is one bit for each of the four channels implemented. This register affects the mapping from application trigger to trigger outputs.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_ctiouten6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_ctiouten6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cti__cfg__cscti_cfg_ctiouten6`]
module"]
#[doc(alias = "CTI__CFG__CSCTI_CFG_CTIOUTEN6")]
pub type Cti_Cfg_CsctiCfgCtiouten6 =
    crate::Reg<cti__cfg__cscti_cfg_ctiouten6::Cti_Cfg_CsctiCfgCtiouten6Spec>;
#[doc = "The CTI Channel to Trigger 6 Enable Registers define which channels can generate a ctitrigout\\[6\\]
output. Within this register there is one bit for each of the four channels implemented. This register affects the mapping from application trigger to trigger outputs."]
pub mod cti__cfg__cscti_cfg_ctiouten6;
#[doc = "CTI__CFG__CSCTI_CFG_CTIOUTEN7 (rw) register accessor: The CTI Channel to Trigger 7 Enable Registers define which channels can generate a ctitrigout\\[7\\]
output. Within this register there is one bit for each of the four channels implemented. This register affects the mapping from application trigger to trigger outputs.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_ctiouten7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_ctiouten7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cti__cfg__cscti_cfg_ctiouten7`]
module"]
#[doc(alias = "CTI__CFG__CSCTI_CFG_CTIOUTEN7")]
pub type Cti_Cfg_CsctiCfgCtiouten7 =
    crate::Reg<cti__cfg__cscti_cfg_ctiouten7::Cti_Cfg_CsctiCfgCtiouten7Spec>;
#[doc = "The CTI Channel to Trigger 7 Enable Registers define which channels can generate a ctitrigout\\[7\\]
output. Within this register there is one bit for each of the four channels implemented. This register affects the mapping from application trigger to trigger outputs."]
pub mod cti__cfg__cscti_cfg_ctiouten7;
#[doc = "CTI__CFG__CSCTI_CFG_CTITRIGINSTATUS (rw) register accessor: The CTI Trigger In Status Register provides the status of the ctitrigin inputs.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_ctitriginstatus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_ctitriginstatus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cti__cfg__cscti_cfg_ctitriginstatus`]
module"]
#[doc(alias = "CTI__CFG__CSCTI_CFG_CTITRIGINSTATUS")]
pub type Cti_Cfg_CsctiCfgCtitriginstatus =
    crate::Reg<cti__cfg__cscti_cfg_ctitriginstatus::Cti_Cfg_CsctiCfgCtitriginstatusSpec>;
#[doc = "The CTI Trigger In Status Register provides the status of the ctitrigin inputs."]
pub mod cti__cfg__cscti_cfg_ctitriginstatus;
#[doc = "CTI__CFG__CSCTI_CFG_CTITRIGOUTSTATUS (rw) register accessor: The CTI Trigger Out Status Register provides the status of the ctitrigout outputs.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_ctitrigoutstatus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_ctitrigoutstatus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cti__cfg__cscti_cfg_ctitrigoutstatus`]
module"]
#[doc(alias = "CTI__CFG__CSCTI_CFG_CTITRIGOUTSTATUS")]
pub type Cti_Cfg_CsctiCfgCtitrigoutstatus =
    crate::Reg<cti__cfg__cscti_cfg_ctitrigoutstatus::Cti_Cfg_CsctiCfgCtitrigoutstatusSpec>;
#[doc = "The CTI Trigger Out Status Register provides the status of the ctitrigout outputs."]
pub mod cti__cfg__cscti_cfg_ctitrigoutstatus;
#[doc = "CTI__CFG__CSCTI_CFG_CTICHINSTATUS (rw) register accessor: The CTI Channel In Status Register provides the status of the ctichin inputs.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_ctichinstatus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_ctichinstatus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cti__cfg__cscti_cfg_ctichinstatus`]
module"]
#[doc(alias = "CTI__CFG__CSCTI_CFG_CTICHINSTATUS")]
pub type Cti_Cfg_CsctiCfgCtichinstatus =
    crate::Reg<cti__cfg__cscti_cfg_ctichinstatus::Cti_Cfg_CsctiCfgCtichinstatusSpec>;
#[doc = "The CTI Channel In Status Register provides the status of the ctichin inputs."]
pub mod cti__cfg__cscti_cfg_ctichinstatus;
#[doc = "CTI__CFG__CSCTI_CFG_CTICHOUTSTATUS (rw) register accessor: The CTI Channel Out Status Register provides the status of the CTI ctichout outputs.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_ctichoutstatus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_ctichoutstatus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cti__cfg__cscti_cfg_ctichoutstatus`]
module"]
#[doc(alias = "CTI__CFG__CSCTI_CFG_CTICHOUTSTATUS")]
pub type Cti_Cfg_CsctiCfgCtichoutstatus =
    crate::Reg<cti__cfg__cscti_cfg_ctichoutstatus::Cti_Cfg_CsctiCfgCtichoutstatusSpec>;
#[doc = "The CTI Channel Out Status Register provides the status of the CTI ctichout outputs."]
pub mod cti__cfg__cscti_cfg_ctichoutstatus;
#[doc = "CTI__CFG__CSCTI_CFG_CTIGATE (rw) register accessor: The Gate Enable Register prevents the channels from propagating through the CTM to other CTIs. This enables local cross-triggering, for example for causing an interrupt when the ETM trigger occurs. It can be used effectively with CTIAPPSET, CTIAPPCLEAR, and CTIAPPPULSE for asserting trigger outputs by asserting channels, without affecting the rest of the system. On reset, this register is 0xF, and channel propagation is enabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_ctigate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_ctigate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cti__cfg__cscti_cfg_ctigate`]
module"]
#[doc(alias = "CTI__CFG__CSCTI_CFG_CTIGATE")]
pub type Cti_Cfg_CsctiCfgCtigate =
    crate::Reg<cti__cfg__cscti_cfg_ctigate::Cti_Cfg_CsctiCfgCtigateSpec>;
#[doc = "The Gate Enable Register prevents the channels from propagating through the CTM to other CTIs. This enables local cross-triggering, for example for causing an interrupt when the ETM trigger occurs. It can be used effectively with CTIAPPSET, CTIAPPCLEAR, and CTIAPPPULSE for asserting trigger outputs by asserting channels, without affecting the rest of the system. On reset, this register is 0xF, and channel propagation is enabled."]
pub mod cti__cfg__cscti_cfg_ctigate;
#[doc = "CTI__CFG__CSCTI_CFG_ASICCTL (rw) register accessor: Implementation-defined ASIC control, value written to the register is output on asicctl\\[7 : 0\\].\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_asicctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_asicctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cti__cfg__cscti_cfg_asicctl`]
module"]
#[doc(alias = "CTI__CFG__CSCTI_CFG_ASICCTL")]
pub type Cti_Cfg_CsctiCfgAsicctl =
    crate::Reg<cti__cfg__cscti_cfg_asicctl::Cti_Cfg_CsctiCfgAsicctlSpec>;
#[doc = "Implementation-defined ASIC control, value written to the register is output on asicctl\\[7 : 0\\]."]
pub mod cti__cfg__cscti_cfg_asicctl;
#[doc = "CTI__CFG__CSCTI_CFG_ITCHINACK (rw) register accessor: This register is a write-only register. It can be used to set the value of the CTCHINACK outputs.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_itchinack::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_itchinack::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cti__cfg__cscti_cfg_itchinack`]
module"]
#[doc(alias = "CTI__CFG__CSCTI_CFG_ITCHINACK")]
pub type Cti_Cfg_CsctiCfgItchinack =
    crate::Reg<cti__cfg__cscti_cfg_itchinack::Cti_Cfg_CsctiCfgItchinackSpec>;
#[doc = "This register is a write-only register. It can be used to set the value of the CTCHINACK outputs."]
pub mod cti__cfg__cscti_cfg_itchinack;
#[doc = "CTI__CFG__CSCTI_CFG_ITTRIGINACK (rw) register accessor: This register is a write-only register. It can be used to set the value of the CTTRIGINACK outputs.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_ittriginack::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_ittriginack::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cti__cfg__cscti_cfg_ittriginack`]
module"]
#[doc(alias = "CTI__CFG__CSCTI_CFG_ITTRIGINACK")]
pub type Cti_Cfg_CsctiCfgIttriginack =
    crate::Reg<cti__cfg__cscti_cfg_ittriginack::Cti_Cfg_CsctiCfgIttriginackSpec>;
#[doc = "This register is a write-only register. It can be used to set the value of the CTTRIGINACK outputs."]
pub mod cti__cfg__cscti_cfg_ittriginack;
#[doc = "CTI__CFG__CSCTI_CFG_ITCHOUT (rw) register accessor: This register is a write-only register. It can be used to set the value of the CTCHOUT outputs.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_itchout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_itchout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cti__cfg__cscti_cfg_itchout`]
module"]
#[doc(alias = "CTI__CFG__CSCTI_CFG_ITCHOUT")]
pub type Cti_Cfg_CsctiCfgItchout =
    crate::Reg<cti__cfg__cscti_cfg_itchout::Cti_Cfg_CsctiCfgItchoutSpec>;
#[doc = "This register is a write-only register. It can be used to set the value of the CTCHOUT outputs."]
pub mod cti__cfg__cscti_cfg_itchout;
#[doc = "CTI__CFG__CSCTI_CFG_ITTRIGOUT (rw) register accessor: This register is a write-only register. It can be used to set the value of the CTTRIGOUT outputs.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_ittrigout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_ittrigout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cti__cfg__cscti_cfg_ittrigout`]
module"]
#[doc(alias = "CTI__CFG__CSCTI_CFG_ITTRIGOUT")]
pub type Cti_Cfg_CsctiCfgIttrigout =
    crate::Reg<cti__cfg__cscti_cfg_ittrigout::Cti_Cfg_CsctiCfgIttrigoutSpec>;
#[doc = "This register is a write-only register. It can be used to set the value of the CTTRIGOUT outputs."]
pub mod cti__cfg__cscti_cfg_ittrigout;
#[doc = "CTI__CFG__CSCTI_CFG_ITCHOUTACK (rw) register accessor: This register is a read-only register. It can be used to read the values of the CTCHOUTACK inputs.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_itchoutack::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_itchoutack::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cti__cfg__cscti_cfg_itchoutack`]
module"]
#[doc(alias = "CTI__CFG__CSCTI_CFG_ITCHOUTACK")]
pub type Cti_Cfg_CsctiCfgItchoutack =
    crate::Reg<cti__cfg__cscti_cfg_itchoutack::Cti_Cfg_CsctiCfgItchoutackSpec>;
#[doc = "This register is a read-only register. It can be used to read the values of the CTCHOUTACK inputs."]
pub mod cti__cfg__cscti_cfg_itchoutack;
#[doc = "CTI__CFG__CSCTI_CFG_ITTRIGOUTACK (rw) register accessor: This register is a read-only register. It can be used to read the values of the CTTRIGOUTACK inputs.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_ittrigoutack::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_ittrigoutack::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cti__cfg__cscti_cfg_ittrigoutack`]
module"]
#[doc(alias = "CTI__CFG__CSCTI_CFG_ITTRIGOUTACK")]
pub type Cti_Cfg_CsctiCfgIttrigoutack =
    crate::Reg<cti__cfg__cscti_cfg_ittrigoutack::Cti_Cfg_CsctiCfgIttrigoutackSpec>;
#[doc = "This register is a read-only register. It can be used to read the values of the CTTRIGOUTACK inputs."]
pub mod cti__cfg__cscti_cfg_ittrigoutack;
#[doc = "CTI__CFG__CSCTI_CFG_ITCHIN (rw) register accessor: This register is a read-only register. It can be used to read the values of the CTCHIN inputs.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_itchin::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_itchin::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cti__cfg__cscti_cfg_itchin`]
module"]
#[doc(alias = "CTI__CFG__CSCTI_CFG_ITCHIN")]
pub type Cti_Cfg_CsctiCfgItchin =
    crate::Reg<cti__cfg__cscti_cfg_itchin::Cti_Cfg_CsctiCfgItchinSpec>;
#[doc = "This register is a read-only register. It can be used to read the values of the CTCHIN inputs."]
pub mod cti__cfg__cscti_cfg_itchin;
#[doc = "CTI__CFG__CSCTI_CFG_ITTRIGIN (rw) register accessor: This register is a read-only register. It can be used to read the values of the CTTRIGIN inputs.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_ittrigin::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_ittrigin::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cti__cfg__cscti_cfg_ittrigin`]
module"]
#[doc(alias = "CTI__CFG__CSCTI_CFG_ITTRIGIN")]
pub type Cti_Cfg_CsctiCfgIttrigin =
    crate::Reg<cti__cfg__cscti_cfg_ittrigin::Cti_Cfg_CsctiCfgIttriginSpec>;
#[doc = "This register is a read-only register. It can be used to read the values of the CTTRIGIN inputs."]
pub mod cti__cfg__cscti_cfg_ittrigin;
#[doc = "CTI__CFG__CSCTI_CFG_ITCTRL (rw) register accessor: This register is used to enable topology detection. For more information see the CoreSight Architecture Specification. This register enables the component to switch from a functional mode, the default behavior, to integration mode where the inputs and outputs of the component can be directly controlled for the purpose of integration testing and topology solving. Note : When a device has been in integration mode, it might not function with the original behavior. After performing integration or topology detection, you must reset the system to ensure correct behavior of CoreSight and other connected system components that are affected by the integration or topology detection.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_itctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_itctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cti__cfg__cscti_cfg_itctrl`]
module"]
#[doc(alias = "CTI__CFG__CSCTI_CFG_ITCTRL")]
pub type Cti_Cfg_CsctiCfgItctrl =
    crate::Reg<cti__cfg__cscti_cfg_itctrl::Cti_Cfg_CsctiCfgItctrlSpec>;
#[doc = "This register is used to enable topology detection. For more information see the CoreSight Architecture Specification. This register enables the component to switch from a functional mode, the default behavior, to integration mode where the inputs and outputs of the component can be directly controlled for the purpose of integration testing and topology solving. Note : When a device has been in integration mode, it might not function with the original behavior. After performing integration or topology detection, you must reset the system to ensure correct behavior of CoreSight and other connected system components that are affected by the integration or topology detection."]
pub mod cti__cfg__cscti_cfg_itctrl;
#[doc = "CTI__CFG__CSCTI_CFG_CLAIMSET (rw) register accessor: This is used in conjunction with Claim Tag Clear Register, CLAIMCLR. This register forms one half of the Claim Tag value. This location allows individual bits to be set, write, and returns the number of bits that can be set, read.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_claimset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_claimset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cti__cfg__cscti_cfg_claimset`]
module"]
#[doc(alias = "CTI__CFG__CSCTI_CFG_CLAIMSET")]
pub type Cti_Cfg_CsctiCfgClaimset =
    crate::Reg<cti__cfg__cscti_cfg_claimset::Cti_Cfg_CsctiCfgClaimsetSpec>;
#[doc = "This is used in conjunction with Claim Tag Clear Register, CLAIMCLR. This register forms one half of the Claim Tag value. This location allows individual bits to be set, write, and returns the number of bits that can be set, read."]
pub mod cti__cfg__cscti_cfg_claimset;
#[doc = "CTI__CFG__CSCTI_CFG_CLAIMCLR (rw) register accessor: This register is used in conjunction with Claim Tag Set Register, CLAIMSET. This register forms one half of the Claim Tag value. This location enables individual bits to be cleared, write, and returns the current Claim Tag value, read.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_claimclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_claimclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cti__cfg__cscti_cfg_claimclr`]
module"]
#[doc(alias = "CTI__CFG__CSCTI_CFG_CLAIMCLR")]
pub type Cti_Cfg_CsctiCfgClaimclr =
    crate::Reg<cti__cfg__cscti_cfg_claimclr::Cti_Cfg_CsctiCfgClaimclrSpec>;
#[doc = "This register is used in conjunction with Claim Tag Set Register, CLAIMSET. This register forms one half of the Claim Tag value. This location enables individual bits to be cleared, write, and returns the current Claim Tag value, read."]
pub mod cti__cfg__cscti_cfg_claimclr;
#[doc = "CTI__CFG__CSCTI_CFG_LAR (rw) register accessor: This is used to enable write access to device registers. External accesses from a debugger (paddrdbg31 = 1) are not subject to the Lock Registers. A debugger does not have to unlock the component in order to write and modify the registers in the component.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_lar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_lar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cti__cfg__cscti_cfg_lar`]
module"]
#[doc(alias = "CTI__CFG__CSCTI_CFG_LAR")]
pub type Cti_Cfg_CsctiCfgLar = crate::Reg<cti__cfg__cscti_cfg_lar::Cti_Cfg_CsctiCfgLarSpec>;
#[doc = "This is used to enable write access to device registers. External accesses from a debugger (paddrdbg31 = 1) are not subject to the Lock Registers. A debugger does not have to unlock the component in order to write and modify the registers in the component."]
pub mod cti__cfg__cscti_cfg_lar;
#[doc = "CTI__CFG__CSCTI_CFG_LSR (rw) register accessor: This indicates the status of the Lock control mechanism. This lock prevents accidental writes by code under debug. When locked, write access is blocked to all registers, except the Lock Access Register. External accesses from a debugger (paddrdbg31 = 1) are not subject to the Lock Registers. This register reads as 0 when read from an external debugger (paddrdbg31 = 1).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_lsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_lsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cti__cfg__cscti_cfg_lsr`]
module"]
#[doc(alias = "CTI__CFG__CSCTI_CFG_LSR")]
pub type Cti_Cfg_CsctiCfgLsr = crate::Reg<cti__cfg__cscti_cfg_lsr::Cti_Cfg_CsctiCfgLsrSpec>;
#[doc = "This indicates the status of the Lock control mechanism. This lock prevents accidental writes by code under debug. When locked, write access is blocked to all registers, except the Lock Access Register. External accesses from a debugger (paddrdbg31 = 1) are not subject to the Lock Registers. This register reads as 0 when read from an external debugger (paddrdbg31 = 1)."]
pub mod cti__cfg__cscti_cfg_lsr;
#[doc = "CTI__CFG__CSCTI_CFG_AUTHSTATUS (rw) register accessor: Reports what functionality is currently permitted by the authentication interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_authstatus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_authstatus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cti__cfg__cscti_cfg_authstatus`]
module"]
#[doc(alias = "CTI__CFG__CSCTI_CFG_AUTHSTATUS")]
pub type Cti_Cfg_CsctiCfgAuthstatus =
    crate::Reg<cti__cfg__cscti_cfg_authstatus::Cti_Cfg_CsctiCfgAuthstatusSpec>;
#[doc = "Reports what functionality is currently permitted by the authentication interface."]
pub mod cti__cfg__cscti_cfg_authstatus;
#[doc = "CTI__CFG__CSCTI_CFG_DEVID (rw) register accessor: This register indicates the capabilities of the CTI.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_devid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_devid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cti__cfg__cscti_cfg_devid`]
module"]
#[doc(alias = "CTI__CFG__CSCTI_CFG_DEVID")]
pub type Cti_Cfg_CsctiCfgDevid = crate::Reg<cti__cfg__cscti_cfg_devid::Cti_Cfg_CsctiCfgDevidSpec>;
#[doc = "This register indicates the capabilities of the CTI."]
pub mod cti__cfg__cscti_cfg_devid;
#[doc = "CTI__CFG__CSCTI_CFG_DEVTYPE (rw) register accessor: It provides a debugger with information about the component when the Part Number field is not recognized. The debugger can then report this information.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_devtype::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_devtype::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cti__cfg__cscti_cfg_devtype`]
module"]
#[doc(alias = "CTI__CFG__CSCTI_CFG_DEVTYPE")]
pub type Cti_Cfg_CsctiCfgDevtype =
    crate::Reg<cti__cfg__cscti_cfg_devtype::Cti_Cfg_CsctiCfgDevtypeSpec>;
#[doc = "It provides a debugger with information about the component when the Part Number field is not recognized. The debugger can then report this information."]
pub mod cti__cfg__cscti_cfg_devtype;
#[doc = "CTI__CFG__CSCTI_CFG_PERIPHID4 (rw) register accessor: Part of the set of Peripheral Identification registers. Contains part of the designer identity and the memory footprint indicator.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_periphid4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_periphid4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cti__cfg__cscti_cfg_periphid4`]
module"]
#[doc(alias = "CTI__CFG__CSCTI_CFG_PERIPHID4")]
pub type Cti_Cfg_CsctiCfgPeriphid4 =
    crate::Reg<cti__cfg__cscti_cfg_periphid4::Cti_Cfg_CsctiCfgPeriphid4Spec>;
#[doc = "Part of the set of Peripheral Identification registers. Contains part of the designer identity and the memory footprint indicator."]
pub mod cti__cfg__cscti_cfg_periphid4;
#[doc = "CTI__CFG__CSCTI_CFG_PERIPHID0 (rw) register accessor: Part of the set of Peripheral Identification registers. Contains part of the designer specific part number.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_periphid0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_periphid0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cti__cfg__cscti_cfg_periphid0`]
module"]
#[doc(alias = "CTI__CFG__CSCTI_CFG_PERIPHID0")]
pub type Cti_Cfg_CsctiCfgPeriphid0 =
    crate::Reg<cti__cfg__cscti_cfg_periphid0::Cti_Cfg_CsctiCfgPeriphid0Spec>;
#[doc = "Part of the set of Peripheral Identification registers. Contains part of the designer specific part number."]
pub mod cti__cfg__cscti_cfg_periphid0;
#[doc = "CTI__CFG__CSCTI_CFG_PERIPHID1 (rw) register accessor: Part of the set of Peripheral Identification registers. Contains part of the designer specific part number and part of the designer identity.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_periphid1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_periphid1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cti__cfg__cscti_cfg_periphid1`]
module"]
#[doc(alias = "CTI__CFG__CSCTI_CFG_PERIPHID1")]
pub type Cti_Cfg_CsctiCfgPeriphid1 =
    crate::Reg<cti__cfg__cscti_cfg_periphid1::Cti_Cfg_CsctiCfgPeriphid1Spec>;
#[doc = "Part of the set of Peripheral Identification registers. Contains part of the designer specific part number and part of the designer identity."]
pub mod cti__cfg__cscti_cfg_periphid1;
#[doc = "CTI__CFG__CSCTI_CFG_PERIPHID2 (rw) register accessor: Part of the set of Peripheral Identification registers. Contains part of the designer identity and the product revision.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_periphid2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_periphid2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cti__cfg__cscti_cfg_periphid2`]
module"]
#[doc(alias = "CTI__CFG__CSCTI_CFG_PERIPHID2")]
pub type Cti_Cfg_CsctiCfgPeriphid2 =
    crate::Reg<cti__cfg__cscti_cfg_periphid2::Cti_Cfg_CsctiCfgPeriphid2Spec>;
#[doc = "Part of the set of Peripheral Identification registers. Contains part of the designer identity and the product revision."]
pub mod cti__cfg__cscti_cfg_periphid2;
#[doc = "CTI__CFG__CSCTI_CFG_PERIPHID3 (rw) register accessor: Part of the set of Peripheral Identification registers. Contains the RevAnd and Customer Modified fields.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_periphid3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_periphid3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cti__cfg__cscti_cfg_periphid3`]
module"]
#[doc(alias = "CTI__CFG__CSCTI_CFG_PERIPHID3")]
pub type Cti_Cfg_CsctiCfgPeriphid3 =
    crate::Reg<cti__cfg__cscti_cfg_periphid3::Cti_Cfg_CsctiCfgPeriphid3Spec>;
#[doc = "Part of the set of Peripheral Identification registers. Contains the RevAnd and Customer Modified fields."]
pub mod cti__cfg__cscti_cfg_periphid3;
#[doc = "CTI__CFG__CSCTI_CFG_COMPID0 (rw) register accessor: Reserved Reserved Reserved A component identification register, that indicates that the identification registers are present.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_compid0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_compid0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cti__cfg__cscti_cfg_compid0`]
module"]
#[doc(alias = "CTI__CFG__CSCTI_CFG_COMPID0")]
pub type Cti_Cfg_CsctiCfgCompid0 =
    crate::Reg<cti__cfg__cscti_cfg_compid0::Cti_Cfg_CsctiCfgCompid0Spec>;
#[doc = "Reserved Reserved Reserved A component identification register, that indicates that the identification registers are present."]
pub mod cti__cfg__cscti_cfg_compid0;
#[doc = "CTI__CFG__CSCTI_CFG_COMPID1 (rw) register accessor: A component identification register, that indicates that the identification registers are present. This register also indicates the component class.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_compid1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_compid1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cti__cfg__cscti_cfg_compid1`]
module"]
#[doc(alias = "CTI__CFG__CSCTI_CFG_COMPID1")]
pub type Cti_Cfg_CsctiCfgCompid1 =
    crate::Reg<cti__cfg__cscti_cfg_compid1::Cti_Cfg_CsctiCfgCompid1Spec>;
#[doc = "A component identification register, that indicates that the identification registers are present. This register also indicates the component class."]
pub mod cti__cfg__cscti_cfg_compid1;
#[doc = "CTI__CFG__CSCTI_CFG_COMPID2 (rw) register accessor: A component identification register, that indicates that the identification registers are present.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_compid2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_compid2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cti__cfg__cscti_cfg_compid2`]
module"]
#[doc(alias = "CTI__CFG__CSCTI_CFG_COMPID2")]
pub type Cti_Cfg_CsctiCfgCompid2 =
    crate::Reg<cti__cfg__cscti_cfg_compid2::Cti_Cfg_CsctiCfgCompid2Spec>;
#[doc = "A component identification register, that indicates that the identification registers are present."]
pub mod cti__cfg__cscti_cfg_compid2;
#[doc = "CTI__CFG__CSCTI_CFG_COMPID3 (rw) register accessor: A component identification register, that indicates that the identification registers are present.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_compid3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_compid3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cti__cfg__cscti_cfg_compid3`]
module"]
#[doc(alias = "CTI__CFG__CSCTI_CFG_COMPID3")]
pub type Cti_Cfg_CsctiCfgCompid3 =
    crate::Reg<cti__cfg__cscti_cfg_compid3::Cti_Cfg_CsctiCfgCompid3Spec>;
#[doc = "A component identification register, that indicates that the identification registers are present."]
pub mod cti__cfg__cscti_cfg_compid3;
