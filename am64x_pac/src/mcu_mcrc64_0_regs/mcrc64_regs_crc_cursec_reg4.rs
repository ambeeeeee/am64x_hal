#[doc = "Register `MCRC64_REGS_CRC_CURSEC_REG4` reader"]
pub type R = crate::R<Mcrc64RegsCrcCursecReg4Spec>;
#[doc = "Register `MCRC64_REGS_CRC_CURSEC_REG4` writer"]
pub type W = crate::W<Mcrc64RegsCrcCursecReg4Spec>;
#[doc = "Field `CRC_CURSEC4` reader - 15:0\\]
In AUTO mode, this register contains the current sector number of which the signature verification fails. The sector counter is a free running up counter. When a sector fails, the erroneous sector number is logged into current sector ID register and the CRC fail interrupt is generated The sector ID register is frozen until it is read and the CRC fail status bit is cleared by CPU. While it is frozen, it does not capture another erroneous sector number. When this condition happens, an overrun interrupt is generated instead. Once the register is read and the CRC fail interrupt flag is cleared it can capture new erroneous sector number. In Semi-CPU mode, this register is used to indicate the sector number for which the compression complete has last happened."]
pub type CrcCursec4R = crate::FieldReader<u16>;
#[doc = "Field `CRC_CURSEC4` writer - 15:0\\]
In AUTO mode, this register contains the current sector number of which the signature verification fails. The sector counter is a free running up counter. When a sector fails, the erroneous sector number is logged into current sector ID register and the CRC fail interrupt is generated The sector ID register is frozen until it is read and the CRC fail status bit is cleared by CPU. While it is frozen, it does not capture another erroneous sector number. When this condition happens, an overrun interrupt is generated instead. Once the register is read and the CRC fail interrupt flag is cleared it can capture new erroneous sector number. In Semi-CPU mode, this register is used to indicate the sector number for which the compression complete has last happened."]
pub type CrcCursec4W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
In AUTO mode, this register contains the current sector number of which the signature verification fails. The sector counter is a free running up counter. When a sector fails, the erroneous sector number is logged into current sector ID register and the CRC fail interrupt is generated The sector ID register is frozen until it is read and the CRC fail status bit is cleared by CPU. While it is frozen, it does not capture another erroneous sector number. When this condition happens, an overrun interrupt is generated instead. Once the register is read and the CRC fail interrupt flag is cleared it can capture new erroneous sector number. In Semi-CPU mode, this register is used to indicate the sector number for which the compression complete has last happened."]
    #[inline(always)]
    pub fn crc_cursec4(&self) -> CrcCursec4R {
        CrcCursec4R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
In AUTO mode, this register contains the current sector number of which the signature verification fails. The sector counter is a free running up counter. When a sector fails, the erroneous sector number is logged into current sector ID register and the CRC fail interrupt is generated The sector ID register is frozen until it is read and the CRC fail status bit is cleared by CPU. While it is frozen, it does not capture another erroneous sector number. When this condition happens, an overrun interrupt is generated instead. Once the register is read and the CRC fail interrupt flag is cleared it can capture new erroneous sector number. In Semi-CPU mode, this register is used to indicate the sector number for which the compression complete has last happened."]
    #[inline(always)]
    #[must_use]
    pub fn crc_cursec4(&mut self) -> CrcCursec4W<Mcrc64RegsCrcCursecReg4Spec> {
        CrcCursec4W::new(self, 0)
    }
}
#[doc = "CRC Current Sector Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_crc_cursec_reg4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_crc_cursec_reg4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mcrc64RegsCrcCursecReg4Spec;
impl crate::RegisterSpec for Mcrc64RegsCrcCursecReg4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcrc64_regs_crc_cursec_reg4::R`](R) reader structure"]
impl crate::Readable for Mcrc64RegsCrcCursecReg4Spec {}
#[doc = "`write(|w| ..)` method takes [`mcrc64_regs_crc_cursec_reg4::W`](W) writer structure"]
impl crate::Writable for Mcrc64RegsCrcCursecReg4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCRC64_REGS_CRC_CURSEC_REG4 to value 0"]
impl crate::Resettable for Mcrc64RegsCrcCursecReg4Spec {
    const RESET_VALUE: u32 = 0;
}
