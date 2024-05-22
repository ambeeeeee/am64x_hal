#[doc = "Register `CFG_TBCTRL` reader"]
pub type R = crate::R<CfgTbctrlSpec>;
#[doc = "Register `CFG_TBCTRL` writer"]
pub type W = crate::W<CfgTbctrlSpec>;
#[doc = "Field `TBEXT` reader - 0:0\\]
The Timebase External bit selects whether the Free Running Counter 0 is clocked by the internal Up Counter 0 or from the external signal NTUx. Since setting the TBEXT bit to 1 resets Up Counter 0, Free Running Counter 0 will not be incremented in this occurence. The only source which is able to increment Free Running Counter 0 is NTUx. When the Timebase Supervisor circuit detects a missing clockedge, then the TBEXT bit is reset. The selection if the external signal should be used, can only be done by software. User and privilege mode (read): 0 = UC0 clocks FRC0 1 = NTUx clocks FRC0 Privilege mode (write): 0 = MUX is switched to internal UC0 clocking scheme 1 = MUX is switched to external NTUx clocking scheme"]
pub type TbextR = crate::BitReader;
#[doc = "Field `TBEXT` writer - 0:0\\]
The Timebase External bit selects whether the Free Running Counter 0 is clocked by the internal Up Counter 0 or from the external signal NTUx. Since setting the TBEXT bit to 1 resets Up Counter 0, Free Running Counter 0 will not be incremented in this occurence. The only source which is able to increment Free Running Counter 0 is NTUx. When the Timebase Supervisor circuit detects a missing clockedge, then the TBEXT bit is reset. The selection if the external signal should be used, can only be done by software. User and privilege mode (read): 0 = UC0 clocks FRC0 1 = NTUx clocks FRC0 Privilege mode (write): 0 = MUX is switched to internal UC0 clocking scheme 1 = MUX is switched to external NTUx clocking scheme"]
pub type TbextW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INC` reader - 1:1\\]
This bit determines wether the Free Running Counter 0 is automatically incremented if a failing clock on the NTUx signal is detected. User and privilege mode (read): 0 = FRC0 will not be incremented 1 = FRC0 will be incremented Privilege mode (write): 0 = Do not increment FRC0 on failing external clock 1 = Increment FRC0 on failing external clock"]
pub type IncR = crate::BitReader;
#[doc = "Field `INC` writer - 1:1\\]
This bit determines wether the Free Running Counter 0 is automatically incremented if a failing clock on the NTUx signal is detected. User and privilege mode (read): 0 = FRC0 will not be incremented 1 = FRC0 will be incremented Privilege mode (write): 0 = Do not increment FRC0 on failing external clock 1 = Increment FRC0 on failing external clock"]
pub type IncW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
The Timebase External bit selects whether the Free Running Counter 0 is clocked by the internal Up Counter 0 or from the external signal NTUx. Since setting the TBEXT bit to 1 resets Up Counter 0, Free Running Counter 0 will not be incremented in this occurence. The only source which is able to increment Free Running Counter 0 is NTUx. When the Timebase Supervisor circuit detects a missing clockedge, then the TBEXT bit is reset. The selection if the external signal should be used, can only be done by software. User and privilege mode (read): 0 = UC0 clocks FRC0 1 = NTUx clocks FRC0 Privilege mode (write): 0 = MUX is switched to internal UC0 clocking scheme 1 = MUX is switched to external NTUx clocking scheme"]
    #[inline(always)]
    pub fn tbext(&self) -> TbextR {
        TbextR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
This bit determines wether the Free Running Counter 0 is automatically incremented if a failing clock on the NTUx signal is detected. User and privilege mode (read): 0 = FRC0 will not be incremented 1 = FRC0 will be incremented Privilege mode (write): 0 = Do not increment FRC0 on failing external clock 1 = Increment FRC0 on failing external clock"]
    #[inline(always)]
    pub fn inc(&self) -> IncR {
        IncR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
The Timebase External bit selects whether the Free Running Counter 0 is clocked by the internal Up Counter 0 or from the external signal NTUx. Since setting the TBEXT bit to 1 resets Up Counter 0, Free Running Counter 0 will not be incremented in this occurence. The only source which is able to increment Free Running Counter 0 is NTUx. When the Timebase Supervisor circuit detects a missing clockedge, then the TBEXT bit is reset. The selection if the external signal should be used, can only be done by software. User and privilege mode (read): 0 = UC0 clocks FRC0 1 = NTUx clocks FRC0 Privilege mode (write): 0 = MUX is switched to internal UC0 clocking scheme 1 = MUX is switched to external NTUx clocking scheme"]
    #[inline(always)]
    #[must_use]
    pub fn tbext(&mut self) -> TbextW<CfgTbctrlSpec> {
        TbextW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
This bit determines wether the Free Running Counter 0 is automatically incremented if a failing clock on the NTUx signal is detected. User and privilege mode (read): 0 = FRC0 will not be incremented 1 = FRC0 will be incremented Privilege mode (write): 0 = Do not increment FRC0 on failing external clock 1 = Increment FRC0 on failing external clock"]
    #[inline(always)]
    #[must_use]
    pub fn inc(&mut self) -> IncW<CfgTbctrlSpec> {
        IncW::new(self, 1)
    }
}
#[doc = "CFG_TBCTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tbctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tbctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgTbctrlSpec;
impl crate::RegisterSpec for CfgTbctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_tbctrl::R`](R) reader structure"]
impl crate::Readable for CfgTbctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_tbctrl::W`](W) writer structure"]
impl crate::Writable for CfgTbctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_TBCTRL to value 0"]
impl crate::Resettable for CfgTbctrlSpec {
    const RESET_VALUE: u32 = 0;
}
