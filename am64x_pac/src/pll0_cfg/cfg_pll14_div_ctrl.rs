#[doc = "Register `CFG_pll14_DIV_CTRL` reader"]
pub type R = crate::R<CfgPll14DivCtrlSpec>;
#[doc = "Register `CFG_pll14_DIV_CTRL` writer"]
pub type W = crate::W<CfgPll14DivCtrlSpec>;
#[doc = "Field `REF_DIV` reader - 5:0\\]
Reference clock pre-divider. Supports values of 1-63 6'b000000 - Reserved /(do not use/) 6'b000001 - Divide by 1 6'b000010 - Divide by 2 : 6'b111111 - Divide by 63"]
pub type RefDivR = crate::FieldReader;
#[doc = "Field `REF_DIV` writer - 5:0\\]
Reference clock pre-divider. Supports values of 1-63 6'b000000 - Reserved /(do not use/) 6'b000001 - Divide by 1 6'b000010 - Divide by 2 : 6'b111111 - Divide by 63"]
pub type RefDivW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `POST_DIV1` reader - 18:16\\]
Primary post divider. To ensure correct operation, post_div1 must always be programmed to a value equal to or greater that post_div2. Supports values of 1-7i Field values (Others are reserved): 3'b000 - Reserved (do not use) 3'b001 - Divide by 1 3'b010 - Divide by 2 3'b011 - Divide by 3 3'b100 - Divide by 4 3'b101 - Divide by 5 3'b110 - Divide by 6 3'b111 - Divide by 7"]
pub type PostDiv1R = crate::FieldReader;
#[doc = "Field `POST_DIV1` writer - 18:16\\]
Primary post divider. To ensure correct operation, post_div1 must always be programmed to a value equal to or greater that post_div2. Supports values of 1-7i Field values (Others are reserved): 3'b000 - Reserved (do not use) 3'b001 - Divide by 1 3'b010 - Divide by 2 3'b011 - Divide by 3 3'b100 - Divide by 4 3'b101 - Divide by 5 3'b110 - Divide by 6 3'b111 - Divide by 7"]
pub type PostDiv1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `POST_DIV2` reader - 26:24\\]
Secondary post divider. Supports values of 1-7"]
pub type PostDiv2R = crate::FieldReader;
#[doc = "Field `POST_DIV2` writer - 26:24\\]
Secondary post divider. Supports values of 1-7"]
pub type PostDiv2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Reference clock pre-divider. Supports values of 1-63 6'b000000 - Reserved /(do not use/) 6'b000001 - Divide by 1 6'b000010 - Divide by 2 : 6'b111111 - Divide by 63"]
    #[inline(always)]
    pub fn ref_div(&self) -> RefDivR {
        RefDivR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Primary post divider. To ensure correct operation, post_div1 must always be programmed to a value equal to or greater that post_div2. Supports values of 1-7i Field values (Others are reserved): 3'b000 - Reserved (do not use) 3'b001 - Divide by 1 3'b010 - Divide by 2 3'b011 - Divide by 3 3'b100 - Divide by 4 3'b101 - Divide by 5 3'b110 - Divide by 6 3'b111 - Divide by 7"]
    #[inline(always)]
    pub fn post_div1(&self) -> PostDiv1R {
        PostDiv1R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Secondary post divider. Supports values of 1-7"]
    #[inline(always)]
    pub fn post_div2(&self) -> PostDiv2R {
        PostDiv2R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Reference clock pre-divider. Supports values of 1-63 6'b000000 - Reserved /(do not use/) 6'b000001 - Divide by 1 6'b000010 - Divide by 2 : 6'b111111 - Divide by 63"]
    #[inline(always)]
    #[must_use]
    pub fn ref_div(&mut self) -> RefDivW<CfgPll14DivCtrlSpec> {
        RefDivW::new(self, 0)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Primary post divider. To ensure correct operation, post_div1 must always be programmed to a value equal to or greater that post_div2. Supports values of 1-7i Field values (Others are reserved): 3'b000 - Reserved (do not use) 3'b001 - Divide by 1 3'b010 - Divide by 2 3'b011 - Divide by 3 3'b100 - Divide by 4 3'b101 - Divide by 5 3'b110 - Divide by 6 3'b111 - Divide by 7"]
    #[inline(always)]
    #[must_use]
    pub fn post_div1(&mut self) -> PostDiv1W<CfgPll14DivCtrlSpec> {
        PostDiv1W::new(self, 16)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Secondary post divider. Supports values of 1-7"]
    #[inline(always)]
    #[must_use]
    pub fn post_div2(&mut self) -> PostDiv2W<CfgPll14DivCtrlSpec> {
        PostDiv2W::new(self, 24)
    }
}
#[doc = "CFG_pll14_DIV_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll14_div_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll14_div_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgPll14DivCtrlSpec;
impl crate::RegisterSpec for CfgPll14DivCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_pll14_div_ctrl::R`](R) reader structure"]
impl crate::Readable for CfgPll14DivCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_pll14_div_ctrl::W`](W) writer structure"]
impl crate::Writable for CfgPll14DivCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_pll14_DIV_CTRL to value 0x0102_0001"]
impl crate::Resettable for CfgPll14DivCtrlSpec {
    const RESET_VALUE: u32 = 0x0102_0001;
}
