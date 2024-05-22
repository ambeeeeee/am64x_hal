#[doc = "Register `PR1_MDIO_V1P7__MDIO__REGS_MDIO_VERSION_REG` reader"]
pub type R = crate::R<Pr1MdioV1p7_Mdio_RegsMdioVersionRegSpec>;
#[doc = "Register `PR1_MDIO_V1P7__MDIO__REGS_MDIO_VERSION_REG` writer"]
pub type W = crate::W<Pr1MdioV1p7_Mdio_RegsMdioVersionRegSpec>;
#[doc = "Field `REVMINOR` reader - 7:0\\]
Minor revision value"]
pub type RevminorR = crate::FieldReader;
#[doc = "Field `REVMINOR` writer - 7:0\\]
Minor revision value"]
pub type RevminorW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REVMAJ` reader - 15:8\\]
Major revision value"]
pub type RevmajR = crate::FieldReader;
#[doc = "Field `REVMAJ` writer - 15:8\\]
Major revision value"]
pub type RevmajW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MODID` reader - 31:16\\]
Module ID"]
pub type ModidR = crate::FieldReader<u16>;
#[doc = "Field `MODID` writer - 31:16\\]
Module ID"]
pub type ModidW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Minor revision value"]
    #[inline(always)]
    pub fn revminor(&self) -> RevminorR {
        RevminorR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Major revision value"]
    #[inline(always)]
    pub fn revmaj(&self) -> RevmajR {
        RevmajR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Module ID"]
    #[inline(always)]
    pub fn modid(&self) -> ModidR {
        ModidR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Minor revision value"]
    #[inline(always)]
    #[must_use]
    pub fn revminor(&mut self) -> RevminorW<Pr1MdioV1p7_Mdio_RegsMdioVersionRegSpec> {
        RevminorW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Major revision value"]
    #[inline(always)]
    #[must_use]
    pub fn revmaj(&mut self) -> RevmajW<Pr1MdioV1p7_Mdio_RegsMdioVersionRegSpec> {
        RevmajW::new(self, 8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Module ID"]
    #[inline(always)]
    #[must_use]
    pub fn modid(&mut self) -> ModidW<Pr1MdioV1p7_Mdio_RegsMdioVersionRegSpec> {
        ModidW::new(self, 16)
    }
}
#[doc = "version_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mdio_v1p7__mdio__regs_mdio_version_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mdio_v1p7__mdio__regs_mdio_version_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MdioV1p7_Mdio_RegsMdioVersionRegSpec;
impl crate::RegisterSpec for Pr1MdioV1p7_Mdio_RegsMdioVersionRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mdio_v1p7__mdio__regs_mdio_version_reg::R`](R) reader structure"]
impl crate::Readable for Pr1MdioV1p7_Mdio_RegsMdioVersionRegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mdio_v1p7__mdio__regs_mdio_version_reg::W`](W) writer structure"]
impl crate::Writable for Pr1MdioV1p7_Mdio_RegsMdioVersionRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MDIO_V1P7__MDIO__REGS_MDIO_VERSION_REG to value 0x0007_0107"]
impl crate::Resettable for Pr1MdioV1p7_Mdio_RegsMdioVersionRegSpec {
    const RESET_VALUE: u32 = 0x0007_0107;
}
