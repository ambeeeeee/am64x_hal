#[doc = "Register `CTI__CFG__CSCTI_CFG_DEVID` reader"]
pub type R = crate::R<Cti_Cfg_CsctiCfgDevidSpec>;
#[doc = "Register `CTI__CFG__CSCTI_CFG_DEVID` writer"]
pub type W = crate::W<Cti_Cfg_CsctiCfgDevidSpec>;
#[doc = "Field `EXTMUXNUM` reader - 4:0\\]
Indicates the number of multiplexing available on Trigger Inputs and Trigger Outputs using asicctl. Default value of 5'b00000 indicating no multiplexing present. Reflects the value of the Verilog define EXTMUXNUM that the user must alter accordingly."]
pub type ExtmuxnumR = crate::FieldReader;
#[doc = "Field `EXTMUXNUM` writer - 4:0\\]
Indicates the number of multiplexing available on Trigger Inputs and Trigger Outputs using asicctl. Default value of 5'b00000 indicating no multiplexing present. Reflects the value of the Verilog define EXTMUXNUM that the user must alter accordingly."]
pub type ExtmuxnumW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `NUMTRIG` reader - 15:8\\]
Number of ECT triggers available."]
pub type NumtrigR = crate::FieldReader;
#[doc = "Field `NUMTRIG` writer - 15:8\\]
Number of ECT triggers available."]
pub type NumtrigW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NUMCH` reader - 19:16\\]
Number of ECT channels available."]
pub type NumchR = crate::FieldReader;
#[doc = "Field `NUMCH` writer - 19:16\\]
Number of ECT channels available."]
pub type NumchW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Indicates the number of multiplexing available on Trigger Inputs and Trigger Outputs using asicctl. Default value of 5'b00000 indicating no multiplexing present. Reflects the value of the Verilog define EXTMUXNUM that the user must alter accordingly."]
    #[inline(always)]
    pub fn extmuxnum(&self) -> ExtmuxnumR {
        ExtmuxnumR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Number of ECT triggers available."]
    #[inline(always)]
    pub fn numtrig(&self) -> NumtrigR {
        NumtrigR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Number of ECT channels available."]
    #[inline(always)]
    pub fn numch(&self) -> NumchR {
        NumchR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Indicates the number of multiplexing available on Trigger Inputs and Trigger Outputs using asicctl. Default value of 5'b00000 indicating no multiplexing present. Reflects the value of the Verilog define EXTMUXNUM that the user must alter accordingly."]
    #[inline(always)]
    #[must_use]
    pub fn extmuxnum(&mut self) -> ExtmuxnumW<Cti_Cfg_CsctiCfgDevidSpec> {
        ExtmuxnumW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Number of ECT triggers available."]
    #[inline(always)]
    #[must_use]
    pub fn numtrig(&mut self) -> NumtrigW<Cti_Cfg_CsctiCfgDevidSpec> {
        NumtrigW::new(self, 8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Number of ECT channels available."]
    #[inline(always)]
    #[must_use]
    pub fn numch(&mut self) -> NumchW<Cti_Cfg_CsctiCfgDevidSpec> {
        NumchW::new(self, 16)
    }
}
#[doc = "This register indicates the capabilities of the CTI.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_devid::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_devid::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cti_Cfg_CsctiCfgDevidSpec;
impl crate::RegisterSpec for Cti_Cfg_CsctiCfgDevidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cti__cfg__cscti_cfg_devid::R`](R) reader structure"]
impl crate::Readable for Cti_Cfg_CsctiCfgDevidSpec {}
#[doc = "`write(|w| ..)` method takes [`cti__cfg__cscti_cfg_devid::W`](W) writer structure"]
impl crate::Writable for Cti_Cfg_CsctiCfgDevidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTI__CFG__CSCTI_CFG_DEVID to value 0x0004_0800"]
impl crate::Resettable for Cti_Cfg_CsctiCfgDevidSpec {
    const RESET_VALUE: u32 = 0x0004_0800;
}
