#[doc = "Register `CFG_HL_HWINFO` reader"]
pub type R = crate::R<CfgHlHwinfoSpec>;
#[doc = "Register `CFG_HL_HWINFO` writer"]
pub type W = crate::W<CfgHlHwinfoSpec>;
#[doc = "Field `USEFIFO` reader - 0:0\\]
Use of a FIFO enable: This bit field indicates if a FIFO is integrated within controller design with its management"]
pub type UsefifoR = crate::BitReader;
#[doc = "Field `USEFIFO` writer - 0:0\\]
Use of a FIFO enable: This bit field indicates if a FIFO is integrated within controller design with its management"]
pub type UsefifoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FFNBYTE` reader - 5:1\\]
FIFO number of byte generic parameter This register defines the value of FFNBYTE generic parameter, only MSB bits from 8 down to 4 are taken into account"]
pub type FfnbyteR = crate::FieldReader;
#[doc = "Field `FFNBYTE` writer - 5:1\\]
FIFO number of byte generic parameter This register defines the value of FFNBYTE generic parameter, only MSB bits from 8 down to 4 are taken into account"]
pub type FfnbyteW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RETMODE` reader - 6:6\\]
This bit field indicates whether the retention mode is supported using the pin PIRFFRET"]
pub type RetmodeR = crate::BitReader;
#[doc = "Field `RETMODE` writer - 6:6\\]
This bit field indicates whether the retention mode is supported using the pin PIRFFRET"]
pub type RetmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD` reader - 31:7\\]
Reserved These bits are initialized to zero, and writes to them are ignored"]
pub type RsvdR = crate::FieldReader<u32>;
#[doc = "Field `RSVD` writer - 31:7\\]
Reserved These bits are initialized to zero, and writes to them are ignored"]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Use of a FIFO enable: This bit field indicates if a FIFO is integrated within controller design with its management"]
    #[inline(always)]
    pub fn usefifo(&self) -> UsefifoR {
        UsefifoR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - 5:1\\]
FIFO number of byte generic parameter This register defines the value of FFNBYTE generic parameter, only MSB bits from 8 down to 4 are taken into account"]
    #[inline(always)]
    pub fn ffnbyte(&self) -> FfnbyteR {
        FfnbyteR::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bit 6 - 6:6\\]
This bit field indicates whether the retention mode is supported using the pin PIRFFRET"]
    #[inline(always)]
    pub fn retmode(&self) -> RetmodeR {
        RetmodeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:31 - 31:7\\]
Reserved These bits are initialized to zero, and writes to them are ignored"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Use of a FIFO enable: This bit field indicates if a FIFO is integrated within controller design with its management"]
    #[inline(always)]
    #[must_use]
    pub fn usefifo(&mut self) -> UsefifoW<CfgHlHwinfoSpec> {
        UsefifoW::new(self, 0)
    }
    #[doc = "Bits 1:5 - 5:1\\]
FIFO number of byte generic parameter This register defines the value of FFNBYTE generic parameter, only MSB bits from 8 down to 4 are taken into account"]
    #[inline(always)]
    #[must_use]
    pub fn ffnbyte(&mut self) -> FfnbyteW<CfgHlHwinfoSpec> {
        FfnbyteW::new(self, 1)
    }
    #[doc = "Bit 6 - 6:6\\]
This bit field indicates whether the retention mode is supported using the pin PIRFFRET"]
    #[inline(always)]
    #[must_use]
    pub fn retmode(&mut self) -> RetmodeW<CfgHlHwinfoSpec> {
        RetmodeW::new(self, 6)
    }
    #[doc = "Bits 7:31 - 31:7\\]
Reserved These bits are initialized to zero, and writes to them are ignored"]
    #[inline(always)]
    #[must_use]
    pub fn rsvd(&mut self) -> RsvdW<CfgHlHwinfoSpec> {
        RsvdW::new(self, 7)
    }
}
#[doc = "Information about the IP module's hardware configuration, i.e. typically the module's HDL generics (if any). Actual field format and encoding is up to the module's designer to decide.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_hl_hwinfo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_hl_hwinfo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgHlHwinfoSpec;
impl crate::RegisterSpec for CfgHlHwinfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_hl_hwinfo::R`](R) reader structure"]
impl crate::Readable for CfgHlHwinfoSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_hl_hwinfo::W`](W) writer structure"]
impl crate::Writable for CfgHlHwinfoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_HL_HWINFO to value 0x09"]
impl crate::Resettable for CfgHlHwinfoSpec {
    const RESET_VALUE: u32 = 0x09;
}
