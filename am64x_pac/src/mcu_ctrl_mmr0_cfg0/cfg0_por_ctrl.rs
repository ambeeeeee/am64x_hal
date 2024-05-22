#[doc = "Register `CFG0_POR_CTRL` reader"]
pub type R = crate::R<Cfg0PorCtrlSpec>;
#[doc = "Register `CFG0_POR_CTRL` writer"]
pub type W = crate::W<Cfg0PorCtrlSpec>;
#[doc = "Field `POR_CTRL_MASK_HHV` reader - 4:4\\]
Mask HHV/SOC_PORz outputs when applying new trim values"]
pub type PorCtrlMaskHhvR = crate::BitReader;
#[doc = "Field `POR_CTRL_MASK_HHV` writer - 4:4\\]
Mask HHV/SOC_PORz outputs when applying new trim values"]
pub type PorCtrlMaskHhvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POR_CTRL_TRIM_SEL` reader - 7:7\\]
POR Trim Select"]
pub type PorCtrlTrimSelR = crate::BitReader;
#[doc = "Field `POR_CTRL_TRIM_SEL` writer - 7:7\\]
POR Trim Select"]
pub type PorCtrlTrimSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POR_CTRL_OVRD0` reader - 16:16\\]
PORHV override active"]
pub type PorCtrlOvrd0R = crate::BitReader;
#[doc = "Field `POR_CTRL_OVRD0` writer - 16:16\\]
PORHV override active"]
pub type PorCtrlOvrd0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POR_CTRL_OVRD1` reader - 17:17\\]
BGOK override active"]
pub type PorCtrlOvrd1R = crate::BitReader;
#[doc = "Field `POR_CTRL_OVRD1` writer - 17:17\\]
BGOK override active"]
pub type PorCtrlOvrd1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POR_CTRL_OVRD2` reader - 18:18\\]
POKHV override active"]
pub type PorCtrlOvrd2R = crate::BitReader;
#[doc = "Field `POR_CTRL_OVRD2` writer - 18:18\\]
POKHV override active"]
pub type PorCtrlOvrd2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POR_CTRL_OVRD3` reader - 19:19\\]
POKLVA override active"]
pub type PorCtrlOvrd3R = crate::BitReader;
#[doc = "Field `POR_CTRL_OVRD3` writer - 19:19\\]
POKLVA override active"]
pub type PorCtrlOvrd3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POR_CTRL_OVRD4` reader - 20:20\\]
POKLVB override active"]
pub type PorCtrlOvrd4R = crate::BitReader;
#[doc = "Field `POR_CTRL_OVRD4` writer - 20:20\\]
POKLVB override active"]
pub type PorCtrlOvrd4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POR_CTRL_OVRD5` reader - 21:21\\]
Reserved override active"]
pub type PorCtrlOvrd5R = crate::BitReader;
#[doc = "Field `POR_CTRL_OVRD5` writer - 21:21\\]
Reserved override active"]
pub type PorCtrlOvrd5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POR_CTRL_OVRD_SET0` reader - 24:24\\]
PORHV override set"]
pub type PorCtrlOvrdSet0R = crate::BitReader;
#[doc = "Field `POR_CTRL_OVRD_SET0` writer - 24:24\\]
PORHV override set"]
pub type PorCtrlOvrdSet0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POR_CTRL_OVRD_SET1` reader - 25:25\\]
BGOK override set"]
pub type PorCtrlOvrdSet1R = crate::BitReader;
#[doc = "Field `POR_CTRL_OVRD_SET1` writer - 25:25\\]
BGOK override set"]
pub type PorCtrlOvrdSet1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POR_CTRL_OVRD_SET2` reader - 26:26\\]
POKHV override set"]
pub type PorCtrlOvrdSet2R = crate::BitReader;
#[doc = "Field `POR_CTRL_OVRD_SET2` writer - 26:26\\]
POKHV override set"]
pub type PorCtrlOvrdSet2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POR_CTRL_OVRD_SET3` reader - 27:27\\]
POKLVA override set"]
pub type PorCtrlOvrdSet3R = crate::BitReader;
#[doc = "Field `POR_CTRL_OVRD_SET3` writer - 27:27\\]
POKLVA override set"]
pub type PorCtrlOvrdSet3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POR_CTRL_OVRD_SET4` reader - 28:28\\]
POKLVB override set"]
pub type PorCtrlOvrdSet4R = crate::BitReader;
#[doc = "Field `POR_CTRL_OVRD_SET4` writer - 28:28\\]
POKLVB override set"]
pub type PorCtrlOvrdSet4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POR_CTRL_OVRD_SET5` reader - 29:29\\]
Reserved override set"]
pub type PorCtrlOvrdSet5R = crate::BitReader;
#[doc = "Field `POR_CTRL_OVRD_SET5` writer - 29:29\\]
Reserved override set"]
pub type PorCtrlOvrdSet5W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - 4:4\\]
Mask HHV/SOC_PORz outputs when applying new trim values"]
    #[inline(always)]
    pub fn por_ctrl_mask_hhv(&self) -> PorCtrlMaskHhvR {
        PorCtrlMaskHhvR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
POR Trim Select"]
    #[inline(always)]
    pub fn por_ctrl_trim_sel(&self) -> PorCtrlTrimSelR {
        PorCtrlTrimSelR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
PORHV override active"]
    #[inline(always)]
    pub fn por_ctrl_ovrd0(&self) -> PorCtrlOvrd0R {
        PorCtrlOvrd0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
BGOK override active"]
    #[inline(always)]
    pub fn por_ctrl_ovrd1(&self) -> PorCtrlOvrd1R {
        PorCtrlOvrd1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
POKHV override active"]
    #[inline(always)]
    pub fn por_ctrl_ovrd2(&self) -> PorCtrlOvrd2R {
        PorCtrlOvrd2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
POKLVA override active"]
    #[inline(always)]
    pub fn por_ctrl_ovrd3(&self) -> PorCtrlOvrd3R {
        PorCtrlOvrd3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
POKLVB override active"]
    #[inline(always)]
    pub fn por_ctrl_ovrd4(&self) -> PorCtrlOvrd4R {
        PorCtrlOvrd4R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Reserved override active"]
    #[inline(always)]
    pub fn por_ctrl_ovrd5(&self) -> PorCtrlOvrd5R {
        PorCtrlOvrd5R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
PORHV override set"]
    #[inline(always)]
    pub fn por_ctrl_ovrd_set0(&self) -> PorCtrlOvrdSet0R {
        PorCtrlOvrdSet0R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
BGOK override set"]
    #[inline(always)]
    pub fn por_ctrl_ovrd_set1(&self) -> PorCtrlOvrdSet1R {
        PorCtrlOvrdSet1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
POKHV override set"]
    #[inline(always)]
    pub fn por_ctrl_ovrd_set2(&self) -> PorCtrlOvrdSet2R {
        PorCtrlOvrdSet2R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
POKLVA override set"]
    #[inline(always)]
    pub fn por_ctrl_ovrd_set3(&self) -> PorCtrlOvrdSet3R {
        PorCtrlOvrdSet3R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
POKLVB override set"]
    #[inline(always)]
    pub fn por_ctrl_ovrd_set4(&self) -> PorCtrlOvrdSet4R {
        PorCtrlOvrdSet4R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Reserved override set"]
    #[inline(always)]
    pub fn por_ctrl_ovrd_set5(&self) -> PorCtrlOvrdSet5R {
        PorCtrlOvrdSet5R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - 4:4\\]
Mask HHV/SOC_PORz outputs when applying new trim values"]
    #[inline(always)]
    #[must_use]
    pub fn por_ctrl_mask_hhv(&mut self) -> PorCtrlMaskHhvW<Cfg0PorCtrlSpec> {
        PorCtrlMaskHhvW::new(self, 4)
    }
    #[doc = "Bit 7 - 7:7\\]
POR Trim Select"]
    #[inline(always)]
    #[must_use]
    pub fn por_ctrl_trim_sel(&mut self) -> PorCtrlTrimSelW<Cfg0PorCtrlSpec> {
        PorCtrlTrimSelW::new(self, 7)
    }
    #[doc = "Bit 16 - 16:16\\]
PORHV override active"]
    #[inline(always)]
    #[must_use]
    pub fn por_ctrl_ovrd0(&mut self) -> PorCtrlOvrd0W<Cfg0PorCtrlSpec> {
        PorCtrlOvrd0W::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
BGOK override active"]
    #[inline(always)]
    #[must_use]
    pub fn por_ctrl_ovrd1(&mut self) -> PorCtrlOvrd1W<Cfg0PorCtrlSpec> {
        PorCtrlOvrd1W::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
POKHV override active"]
    #[inline(always)]
    #[must_use]
    pub fn por_ctrl_ovrd2(&mut self) -> PorCtrlOvrd2W<Cfg0PorCtrlSpec> {
        PorCtrlOvrd2W::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
POKLVA override active"]
    #[inline(always)]
    #[must_use]
    pub fn por_ctrl_ovrd3(&mut self) -> PorCtrlOvrd3W<Cfg0PorCtrlSpec> {
        PorCtrlOvrd3W::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
POKLVB override active"]
    #[inline(always)]
    #[must_use]
    pub fn por_ctrl_ovrd4(&mut self) -> PorCtrlOvrd4W<Cfg0PorCtrlSpec> {
        PorCtrlOvrd4W::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
Reserved override active"]
    #[inline(always)]
    #[must_use]
    pub fn por_ctrl_ovrd5(&mut self) -> PorCtrlOvrd5W<Cfg0PorCtrlSpec> {
        PorCtrlOvrd5W::new(self, 21)
    }
    #[doc = "Bit 24 - 24:24\\]
PORHV override set"]
    #[inline(always)]
    #[must_use]
    pub fn por_ctrl_ovrd_set0(&mut self) -> PorCtrlOvrdSet0W<Cfg0PorCtrlSpec> {
        PorCtrlOvrdSet0W::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
BGOK override set"]
    #[inline(always)]
    #[must_use]
    pub fn por_ctrl_ovrd_set1(&mut self) -> PorCtrlOvrdSet1W<Cfg0PorCtrlSpec> {
        PorCtrlOvrdSet1W::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
POKHV override set"]
    #[inline(always)]
    #[must_use]
    pub fn por_ctrl_ovrd_set2(&mut self) -> PorCtrlOvrdSet2W<Cfg0PorCtrlSpec> {
        PorCtrlOvrdSet2W::new(self, 26)
    }
    #[doc = "Bit 27 - 27:27\\]
POKLVA override set"]
    #[inline(always)]
    #[must_use]
    pub fn por_ctrl_ovrd_set3(&mut self) -> PorCtrlOvrdSet3W<Cfg0PorCtrlSpec> {
        PorCtrlOvrdSet3W::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
POKLVB override set"]
    #[inline(always)]
    #[must_use]
    pub fn por_ctrl_ovrd_set4(&mut self) -> PorCtrlOvrdSet4W<Cfg0PorCtrlSpec> {
        PorCtrlOvrdSet4W::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
Reserved override set"]
    #[inline(always)]
    #[must_use]
    pub fn por_ctrl_ovrd_set5(&mut self) -> PorCtrlOvrdSet5W<Cfg0PorCtrlSpec> {
        PorCtrlOvrdSet5W::new(self, 29)
    }
}
#[doc = "CFG0_POR_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_por_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_por_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0PorCtrlSpec;
impl crate::RegisterSpec for Cfg0PorCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_por_ctrl::R`](R) reader structure"]
impl crate::Readable for Cfg0PorCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_por_ctrl::W`](W) writer structure"]
impl crate::Writable for Cfg0PorCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_POR_CTRL to value 0x10"]
impl crate::Resettable for Cfg0PorCtrlSpec {
    const RESET_VALUE: u32 = 0x10;
}
