#[doc = "Register `CFG_GPMC_ECC_RESULT` reader"]
pub type R = crate::R<CfgGpmcEccResultSpec>;
#[doc = "Register `CFG_GPMC_ECC_RESULT` writer"]
pub type W = crate::W<CfgGpmcEccResultSpec>;
#[doc = "Field `P1E` reader - 0:0\\]
Even Column Parity bit 1"]
pub type P1eR = crate::BitReader;
#[doc = "Field `P1E` writer - 0:0\\]
Even Column Parity bit 1"]
pub type P1eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2E` reader - 1:1\\]
Even Column Parity bit 2"]
pub type P2eR = crate::BitReader;
#[doc = "Field `P2E` writer - 1:1\\]
Even Column Parity bit 2"]
pub type P2eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4E` reader - 2:2\\]
Even Column Parity bit 4"]
pub type P4eR = crate::BitReader;
#[doc = "Field `P4E` writer - 2:2\\]
Even Column Parity bit 4"]
pub type P4eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8E` reader - 3:3\\]
Even Row Parity bit 8"]
pub type P8eR = crate::BitReader;
#[doc = "Field `P8E` writer - 3:3\\]
Even Row Parity bit 8"]
pub type P8eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P16E` reader - 4:4\\]
Even Row Parity bit 16"]
pub type P16eR = crate::BitReader;
#[doc = "Field `P16E` writer - 4:4\\]
Even Row Parity bit 16"]
pub type P16eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P32E` reader - 5:5\\]
Even Row Parity bit 32"]
pub type P32eR = crate::BitReader;
#[doc = "Field `P32E` writer - 5:5\\]
Even Row Parity bit 32"]
pub type P32eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P64E` reader - 6:6\\]
Even Row Parity bit 64"]
pub type P64eR = crate::BitReader;
#[doc = "Field `P64E` writer - 6:6\\]
Even Row Parity bit 64"]
pub type P64eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P128E` reader - 7:7\\]
Even Row Parity bit 128"]
pub type P128eR = crate::BitReader;
#[doc = "Field `P128E` writer - 7:7\\]
Even Row Parity bit 128"]
pub type P128eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P256E` reader - 8:8\\]
Even Row Parity bit 256"]
pub type P256eR = crate::BitReader;
#[doc = "Field `P256E` writer - 8:8\\]
Even Row Parity bit 256"]
pub type P256eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P512E` reader - 9:9\\]
Even Row Parity bit 512"]
pub type P512eR = crate::BitReader;
#[doc = "Field `P512E` writer - 9:9\\]
Even Row Parity bit 512"]
pub type P512eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1024E` reader - 10:10\\]
Even Row Parity bit 1024"]
pub type P1024eR = crate::BitReader;
#[doc = "Field `P1024E` writer - 10:10\\]
Even Row Parity bit 1024"]
pub type P1024eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2048E` reader - 11:11\\]
Even Row Parity bit 2048, only used for ECC computed on 512 Bytes"]
pub type P2048eR = crate::BitReader;
#[doc = "Field `P2048E` writer - 11:11\\]
Even Row Parity bit 2048, only used for ECC computed on 512 Bytes"]
pub type P2048eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1O` reader - 16:16\\]
Odd Column Parity bit 1"]
pub type P1oR = crate::BitReader;
#[doc = "Field `P1O` writer - 16:16\\]
Odd Column Parity bit 1"]
pub type P1oW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2O` reader - 17:17\\]
Odd Column Parity bit 2"]
pub type P2oR = crate::BitReader;
#[doc = "Field `P2O` writer - 17:17\\]
Odd Column Parity bit 2"]
pub type P2oW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4O` reader - 18:18\\]
Odd Column Parity bit 4"]
pub type P4oR = crate::BitReader;
#[doc = "Field `P4O` writer - 18:18\\]
Odd Column Parity bit 4"]
pub type P4oW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8O` reader - 19:19\\]
Odd Row Parity bit 8"]
pub type P8oR = crate::BitReader;
#[doc = "Field `P8O` writer - 19:19\\]
Odd Row Parity bit 8"]
pub type P8oW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P16O` reader - 20:20\\]
Odd Row Parity bit 16"]
pub type P16oR = crate::BitReader;
#[doc = "Field `P16O` writer - 20:20\\]
Odd Row Parity bit 16"]
pub type P16oW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P32O` reader - 21:21\\]
Odd Row Parity bit 32"]
pub type P32oR = crate::BitReader;
#[doc = "Field `P32O` writer - 21:21\\]
Odd Row Parity bit 32"]
pub type P32oW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P64O` reader - 22:22\\]
Odd Row Parity bit 64"]
pub type P64oR = crate::BitReader;
#[doc = "Field `P64O` writer - 22:22\\]
Odd Row Parity bit 64"]
pub type P64oW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P128O` reader - 23:23\\]
Odd Row Parity bit 128"]
pub type P128oR = crate::BitReader;
#[doc = "Field `P128O` writer - 23:23\\]
Odd Row Parity bit 128"]
pub type P128oW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P256O` reader - 24:24\\]
Odd Row Parity bit 256"]
pub type P256oR = crate::BitReader;
#[doc = "Field `P256O` writer - 24:24\\]
Odd Row Parity bit 256"]
pub type P256oW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P512O` reader - 25:25\\]
Odd Row Parity bit 512"]
pub type P512oR = crate::BitReader;
#[doc = "Field `P512O` writer - 25:25\\]
Odd Row Parity bit 512"]
pub type P512oW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1024O` reader - 26:26\\]
Odd Row Parity bit 1024"]
pub type P1024oR = crate::BitReader;
#[doc = "Field `P1024O` writer - 26:26\\]
Odd Row Parity bit 1024"]
pub type P1024oW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2048O` reader - 27:27\\]
Odd Row Parity bit 2048, only used for ECC computed on 512 Bytes"]
pub type P2048oR = crate::BitReader;
#[doc = "Field `P2048O` writer - 27:27\\]
Odd Row Parity bit 2048, only used for ECC computed on 512 Bytes"]
pub type P2048oW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Even Column Parity bit 1"]
    #[inline(always)]
    pub fn p1e(&self) -> P1eR {
        P1eR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Even Column Parity bit 2"]
    #[inline(always)]
    pub fn p2e(&self) -> P2eR {
        P2eR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Even Column Parity bit 4"]
    #[inline(always)]
    pub fn p4e(&self) -> P4eR {
        P4eR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Even Row Parity bit 8"]
    #[inline(always)]
    pub fn p8e(&self) -> P8eR {
        P8eR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Even Row Parity bit 16"]
    #[inline(always)]
    pub fn p16e(&self) -> P16eR {
        P16eR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Even Row Parity bit 32"]
    #[inline(always)]
    pub fn p32e(&self) -> P32eR {
        P32eR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Even Row Parity bit 64"]
    #[inline(always)]
    pub fn p64e(&self) -> P64eR {
        P64eR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Even Row Parity bit 128"]
    #[inline(always)]
    pub fn p128e(&self) -> P128eR {
        P128eR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Even Row Parity bit 256"]
    #[inline(always)]
    pub fn p256e(&self) -> P256eR {
        P256eR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Even Row Parity bit 512"]
    #[inline(always)]
    pub fn p512e(&self) -> P512eR {
        P512eR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Even Row Parity bit 1024"]
    #[inline(always)]
    pub fn p1024e(&self) -> P1024eR {
        P1024eR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Even Row Parity bit 2048, only used for ECC computed on 512 Bytes"]
    #[inline(always)]
    pub fn p2048e(&self) -> P2048eR {
        P2048eR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Odd Column Parity bit 1"]
    #[inline(always)]
    pub fn p1o(&self) -> P1oR {
        P1oR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Odd Column Parity bit 2"]
    #[inline(always)]
    pub fn p2o(&self) -> P2oR {
        P2oR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Odd Column Parity bit 4"]
    #[inline(always)]
    pub fn p4o(&self) -> P4oR {
        P4oR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Odd Row Parity bit 8"]
    #[inline(always)]
    pub fn p8o(&self) -> P8oR {
        P8oR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Odd Row Parity bit 16"]
    #[inline(always)]
    pub fn p16o(&self) -> P16oR {
        P16oR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Odd Row Parity bit 32"]
    #[inline(always)]
    pub fn p32o(&self) -> P32oR {
        P32oR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Odd Row Parity bit 64"]
    #[inline(always)]
    pub fn p64o(&self) -> P64oR {
        P64oR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Odd Row Parity bit 128"]
    #[inline(always)]
    pub fn p128o(&self) -> P128oR {
        P128oR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Odd Row Parity bit 256"]
    #[inline(always)]
    pub fn p256o(&self) -> P256oR {
        P256oR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Odd Row Parity bit 512"]
    #[inline(always)]
    pub fn p512o(&self) -> P512oR {
        P512oR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Odd Row Parity bit 1024"]
    #[inline(always)]
    pub fn p1024o(&self) -> P1024oR {
        P1024oR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Odd Row Parity bit 2048, only used for ECC computed on 512 Bytes"]
    #[inline(always)]
    pub fn p2048o(&self) -> P2048oR {
        P2048oR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Even Column Parity bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn p1e(&mut self) -> P1eW<CfgGpmcEccResultSpec> {
        P1eW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Even Column Parity bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn p2e(&mut self) -> P2eW<CfgGpmcEccResultSpec> {
        P2eW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Even Column Parity bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn p4e(&mut self) -> P4eW<CfgGpmcEccResultSpec> {
        P4eW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Even Row Parity bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn p8e(&mut self) -> P8eW<CfgGpmcEccResultSpec> {
        P8eW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Even Row Parity bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn p16e(&mut self) -> P16eW<CfgGpmcEccResultSpec> {
        P16eW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Even Row Parity bit 32"]
    #[inline(always)]
    #[must_use]
    pub fn p32e(&mut self) -> P32eW<CfgGpmcEccResultSpec> {
        P32eW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Even Row Parity bit 64"]
    #[inline(always)]
    #[must_use]
    pub fn p64e(&mut self) -> P64eW<CfgGpmcEccResultSpec> {
        P64eW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Even Row Parity bit 128"]
    #[inline(always)]
    #[must_use]
    pub fn p128e(&mut self) -> P128eW<CfgGpmcEccResultSpec> {
        P128eW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Even Row Parity bit 256"]
    #[inline(always)]
    #[must_use]
    pub fn p256e(&mut self) -> P256eW<CfgGpmcEccResultSpec> {
        P256eW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Even Row Parity bit 512"]
    #[inline(always)]
    #[must_use]
    pub fn p512e(&mut self) -> P512eW<CfgGpmcEccResultSpec> {
        P512eW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Even Row Parity bit 1024"]
    #[inline(always)]
    #[must_use]
    pub fn p1024e(&mut self) -> P1024eW<CfgGpmcEccResultSpec> {
        P1024eW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Even Row Parity bit 2048, only used for ECC computed on 512 Bytes"]
    #[inline(always)]
    #[must_use]
    pub fn p2048e(&mut self) -> P2048eW<CfgGpmcEccResultSpec> {
        P2048eW::new(self, 11)
    }
    #[doc = "Bit 16 - 16:16\\]
Odd Column Parity bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn p1o(&mut self) -> P1oW<CfgGpmcEccResultSpec> {
        P1oW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Odd Column Parity bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn p2o(&mut self) -> P2oW<CfgGpmcEccResultSpec> {
        P2oW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Odd Column Parity bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn p4o(&mut self) -> P4oW<CfgGpmcEccResultSpec> {
        P4oW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Odd Row Parity bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn p8o(&mut self) -> P8oW<CfgGpmcEccResultSpec> {
        P8oW::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
Odd Row Parity bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn p16o(&mut self) -> P16oW<CfgGpmcEccResultSpec> {
        P16oW::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
Odd Row Parity bit 32"]
    #[inline(always)]
    #[must_use]
    pub fn p32o(&mut self) -> P32oW<CfgGpmcEccResultSpec> {
        P32oW::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
Odd Row Parity bit 64"]
    #[inline(always)]
    #[must_use]
    pub fn p64o(&mut self) -> P64oW<CfgGpmcEccResultSpec> {
        P64oW::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
Odd Row Parity bit 128"]
    #[inline(always)]
    #[must_use]
    pub fn p128o(&mut self) -> P128oW<CfgGpmcEccResultSpec> {
        P128oW::new(self, 23)
    }
    #[doc = "Bit 24 - 24:24\\]
Odd Row Parity bit 256"]
    #[inline(always)]
    #[must_use]
    pub fn p256o(&mut self) -> P256oW<CfgGpmcEccResultSpec> {
        P256oW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Odd Row Parity bit 512"]
    #[inline(always)]
    #[must_use]
    pub fn p512o(&mut self) -> P512oW<CfgGpmcEccResultSpec> {
        P512oW::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
Odd Row Parity bit 1024"]
    #[inline(always)]
    #[must_use]
    pub fn p1024o(&mut self) -> P1024oW<CfgGpmcEccResultSpec> {
        P1024oW::new(self, 26)
    }
    #[doc = "Bit 27 - 27:27\\]
Odd Row Parity bit 2048, only used for ECC computed on 512 Bytes"]
    #[inline(always)]
    #[must_use]
    pub fn p2048o(&mut self) -> P2048oW<CfgGpmcEccResultSpec> {
        P2048oW::new(self, 27)
    }
}
#[doc = "ECC result register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_ecc_result::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_ecc_result::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgGpmcEccResultSpec;
impl crate::RegisterSpec for CfgGpmcEccResultSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_gpmc_ecc_result::R`](R) reader structure"]
impl crate::Readable for CfgGpmcEccResultSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_gpmc_ecc_result::W`](W) writer structure"]
impl crate::Writable for CfgGpmcEccResultSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_GPMC_ECC_RESULT to value 0"]
impl crate::Resettable for CfgGpmcEccResultSpec {
    const RESET_VALUE: u32 = 0;
}
