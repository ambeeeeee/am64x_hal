#[doc = "Register `FW_REGS_br_scrm_64b_clk2_to_scrp_misc_clk2_l0_fw_region_8_permission_0` reader"]
pub type R = crate::R<FwRegsBrScrm64bClk2ToScrpMiscClk2L0FwRegion8Permission0Spec>;
#[doc = "Register `FW_REGS_br_scrm_64b_clk2_to_scrp_misc_clk2_l0_fw_region_8_permission_0` writer"]
pub type W = crate::W<FwRegsBrScrm64bClk2ToScrpMiscClk2L0FwRegion8Permission0Spec>;
#[doc = "Field `SEC_SUPV_WRITE` reader - 0:0\\]
Secure supervisor write allowed."]
pub type SecSupvWriteR = crate::BitReader;
#[doc = "Field `SEC_SUPV_WRITE` writer - 0:0\\]
Secure supervisor write allowed."]
pub type SecSupvWriteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC_SUPV_READ` reader - 1:1\\]
Secure supervisor read allowed."]
pub type SecSupvReadR = crate::BitReader;
#[doc = "Field `SEC_SUPV_READ` writer - 1:1\\]
Secure supervisor read allowed."]
pub type SecSupvReadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC_SUPV_CACHEABLE` reader - 2:2\\]
Secure supervisor cacheable allowed."]
pub type SecSupvCacheableR = crate::BitReader;
#[doc = "Field `SEC_SUPV_CACHEABLE` writer - 2:2\\]
Secure supervisor cacheable allowed."]
pub type SecSupvCacheableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC_SUPV_DEBUG` reader - 3:3\\]
Secure supervisor debug allowed."]
pub type SecSupvDebugR = crate::BitReader;
#[doc = "Field `SEC_SUPV_DEBUG` writer - 3:3\\]
Secure supervisor debug allowed."]
pub type SecSupvDebugW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC_USER_WRITE` reader - 4:4\\]
Secure user write allowed."]
pub type SecUserWriteR = crate::BitReader;
#[doc = "Field `SEC_USER_WRITE` writer - 4:4\\]
Secure user write allowed."]
pub type SecUserWriteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC_USER_READ` reader - 5:5\\]
Secure user read allowed."]
pub type SecUserReadR = crate::BitReader;
#[doc = "Field `SEC_USER_READ` writer - 5:5\\]
Secure user read allowed."]
pub type SecUserReadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC_USER_CACHEABLE` reader - 6:6\\]
Secure user cacheable allowed."]
pub type SecUserCacheableR = crate::BitReader;
#[doc = "Field `SEC_USER_CACHEABLE` writer - 6:6\\]
Secure user cacheable allowed."]
pub type SecUserCacheableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC_USER_DEBUG` reader - 7:7\\]
Secure user debug allowed."]
pub type SecUserDebugR = crate::BitReader;
#[doc = "Field `SEC_USER_DEBUG` writer - 7:7\\]
Secure user debug allowed."]
pub type SecUserDebugW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NONSEC_SUPV_WRITE` reader - 8:8\\]
Non-secure supervisor write allowed."]
pub type NonsecSupvWriteR = crate::BitReader;
#[doc = "Field `NONSEC_SUPV_WRITE` writer - 8:8\\]
Non-secure supervisor write allowed."]
pub type NonsecSupvWriteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NONSEC_SUPV_READ` reader - 9:9\\]
Non-secure supervisor read allowed."]
pub type NonsecSupvReadR = crate::BitReader;
#[doc = "Field `NONSEC_SUPV_READ` writer - 9:9\\]
Non-secure supervisor read allowed."]
pub type NonsecSupvReadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NONSEC_SUPV_CACHEABLE` reader - 10:10\\]
Non-secure supervisor cacheable allowed."]
pub type NonsecSupvCacheableR = crate::BitReader;
#[doc = "Field `NONSEC_SUPV_CACHEABLE` writer - 10:10\\]
Non-secure supervisor cacheable allowed."]
pub type NonsecSupvCacheableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NONSEC_SUPV_DEBUG` reader - 11:11\\]
Non-secure supervisor debug allowed."]
pub type NonsecSupvDebugR = crate::BitReader;
#[doc = "Field `NONSEC_SUPV_DEBUG` writer - 11:11\\]
Non-secure supervisor debug allowed."]
pub type NonsecSupvDebugW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NONSEC_USER_WRITE` reader - 12:12\\]
Non-secure user write allowed."]
pub type NonsecUserWriteR = crate::BitReader;
#[doc = "Field `NONSEC_USER_WRITE` writer - 12:12\\]
Non-secure user write allowed."]
pub type NonsecUserWriteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NONSEC_USER_READ` reader - 13:13\\]
Non-secure user read allowed."]
pub type NonsecUserReadR = crate::BitReader;
#[doc = "Field `NONSEC_USER_READ` writer - 13:13\\]
Non-secure user read allowed."]
pub type NonsecUserReadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NONSEC_USER_CACHEABLE` reader - 14:14\\]
Non-secure user cacheable allowed."]
pub type NonsecUserCacheableR = crate::BitReader;
#[doc = "Field `NONSEC_USER_CACHEABLE` writer - 14:14\\]
Non-secure user cacheable allowed."]
pub type NonsecUserCacheableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NONSEC_USER_DEBUG` reader - 15:15\\]
Non-secure user debug allowed."]
pub type NonsecUserDebugR = crate::BitReader;
#[doc = "Field `NONSEC_USER_DEBUG` writer - 15:15\\]
Non-secure user debug allowed."]
pub type NonsecUserDebugW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV_ID` reader - 23:16\\]
Allowed privid."]
pub type PrivIdR = crate::FieldReader;
#[doc = "Field `PRIV_ID` writer - 23:16\\]
Allowed privid."]
pub type PrivIdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Secure supervisor write allowed."]
    #[inline(always)]
    pub fn sec_supv_write(&self) -> SecSupvWriteR {
        SecSupvWriteR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Secure supervisor read allowed."]
    #[inline(always)]
    pub fn sec_supv_read(&self) -> SecSupvReadR {
        SecSupvReadR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Secure supervisor cacheable allowed."]
    #[inline(always)]
    pub fn sec_supv_cacheable(&self) -> SecSupvCacheableR {
        SecSupvCacheableR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Secure supervisor debug allowed."]
    #[inline(always)]
    pub fn sec_supv_debug(&self) -> SecSupvDebugR {
        SecSupvDebugR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Secure user write allowed."]
    #[inline(always)]
    pub fn sec_user_write(&self) -> SecUserWriteR {
        SecUserWriteR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Secure user read allowed."]
    #[inline(always)]
    pub fn sec_user_read(&self) -> SecUserReadR {
        SecUserReadR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Secure user cacheable allowed."]
    #[inline(always)]
    pub fn sec_user_cacheable(&self) -> SecUserCacheableR {
        SecUserCacheableR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Secure user debug allowed."]
    #[inline(always)]
    pub fn sec_user_debug(&self) -> SecUserDebugR {
        SecUserDebugR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Non-secure supervisor write allowed."]
    #[inline(always)]
    pub fn nonsec_supv_write(&self) -> NonsecSupvWriteR {
        NonsecSupvWriteR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Non-secure supervisor read allowed."]
    #[inline(always)]
    pub fn nonsec_supv_read(&self) -> NonsecSupvReadR {
        NonsecSupvReadR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Non-secure supervisor cacheable allowed."]
    #[inline(always)]
    pub fn nonsec_supv_cacheable(&self) -> NonsecSupvCacheableR {
        NonsecSupvCacheableR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Non-secure supervisor debug allowed."]
    #[inline(always)]
    pub fn nonsec_supv_debug(&self) -> NonsecSupvDebugR {
        NonsecSupvDebugR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Non-secure user write allowed."]
    #[inline(always)]
    pub fn nonsec_user_write(&self) -> NonsecUserWriteR {
        NonsecUserWriteR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Non-secure user read allowed."]
    #[inline(always)]
    pub fn nonsec_user_read(&self) -> NonsecUserReadR {
        NonsecUserReadR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Non-secure user cacheable allowed."]
    #[inline(always)]
    pub fn nonsec_user_cacheable(&self) -> NonsecUserCacheableR {
        NonsecUserCacheableR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Non-secure user debug allowed."]
    #[inline(always)]
    pub fn nonsec_user_debug(&self) -> NonsecUserDebugR {
        NonsecUserDebugR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Allowed privid."]
    #[inline(always)]
    pub fn priv_id(&self) -> PrivIdR {
        PrivIdR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Secure supervisor write allowed."]
    #[inline(always)]
    #[must_use]
    pub fn sec_supv_write(
        &mut self,
    ) -> SecSupvWriteW<FwRegsBrScrm64bClk2ToScrpMiscClk2L0FwRegion8Permission0Spec> {
        SecSupvWriteW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Secure supervisor read allowed."]
    #[inline(always)]
    #[must_use]
    pub fn sec_supv_read(
        &mut self,
    ) -> SecSupvReadW<FwRegsBrScrm64bClk2ToScrpMiscClk2L0FwRegion8Permission0Spec> {
        SecSupvReadW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Secure supervisor cacheable allowed."]
    #[inline(always)]
    #[must_use]
    pub fn sec_supv_cacheable(
        &mut self,
    ) -> SecSupvCacheableW<FwRegsBrScrm64bClk2ToScrpMiscClk2L0FwRegion8Permission0Spec> {
        SecSupvCacheableW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Secure supervisor debug allowed."]
    #[inline(always)]
    #[must_use]
    pub fn sec_supv_debug(
        &mut self,
    ) -> SecSupvDebugW<FwRegsBrScrm64bClk2ToScrpMiscClk2L0FwRegion8Permission0Spec> {
        SecSupvDebugW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Secure user write allowed."]
    #[inline(always)]
    #[must_use]
    pub fn sec_user_write(
        &mut self,
    ) -> SecUserWriteW<FwRegsBrScrm64bClk2ToScrpMiscClk2L0FwRegion8Permission0Spec> {
        SecUserWriteW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Secure user read allowed."]
    #[inline(always)]
    #[must_use]
    pub fn sec_user_read(
        &mut self,
    ) -> SecUserReadW<FwRegsBrScrm64bClk2ToScrpMiscClk2L0FwRegion8Permission0Spec> {
        SecUserReadW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Secure user cacheable allowed."]
    #[inline(always)]
    #[must_use]
    pub fn sec_user_cacheable(
        &mut self,
    ) -> SecUserCacheableW<FwRegsBrScrm64bClk2ToScrpMiscClk2L0FwRegion8Permission0Spec> {
        SecUserCacheableW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Secure user debug allowed."]
    #[inline(always)]
    #[must_use]
    pub fn sec_user_debug(
        &mut self,
    ) -> SecUserDebugW<FwRegsBrScrm64bClk2ToScrpMiscClk2L0FwRegion8Permission0Spec> {
        SecUserDebugW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Non-secure supervisor write allowed."]
    #[inline(always)]
    #[must_use]
    pub fn nonsec_supv_write(
        &mut self,
    ) -> NonsecSupvWriteW<FwRegsBrScrm64bClk2ToScrpMiscClk2L0FwRegion8Permission0Spec> {
        NonsecSupvWriteW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Non-secure supervisor read allowed."]
    #[inline(always)]
    #[must_use]
    pub fn nonsec_supv_read(
        &mut self,
    ) -> NonsecSupvReadW<FwRegsBrScrm64bClk2ToScrpMiscClk2L0FwRegion8Permission0Spec> {
        NonsecSupvReadW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Non-secure supervisor cacheable allowed."]
    #[inline(always)]
    #[must_use]
    pub fn nonsec_supv_cacheable(
        &mut self,
    ) -> NonsecSupvCacheableW<FwRegsBrScrm64bClk2ToScrpMiscClk2L0FwRegion8Permission0Spec> {
        NonsecSupvCacheableW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Non-secure supervisor debug allowed."]
    #[inline(always)]
    #[must_use]
    pub fn nonsec_supv_debug(
        &mut self,
    ) -> NonsecSupvDebugW<FwRegsBrScrm64bClk2ToScrpMiscClk2L0FwRegion8Permission0Spec> {
        NonsecSupvDebugW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Non-secure user write allowed."]
    #[inline(always)]
    #[must_use]
    pub fn nonsec_user_write(
        &mut self,
    ) -> NonsecUserWriteW<FwRegsBrScrm64bClk2ToScrpMiscClk2L0FwRegion8Permission0Spec> {
        NonsecUserWriteW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Non-secure user read allowed."]
    #[inline(always)]
    #[must_use]
    pub fn nonsec_user_read(
        &mut self,
    ) -> NonsecUserReadW<FwRegsBrScrm64bClk2ToScrpMiscClk2L0FwRegion8Permission0Spec> {
        NonsecUserReadW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Non-secure user cacheable allowed."]
    #[inline(always)]
    #[must_use]
    pub fn nonsec_user_cacheable(
        &mut self,
    ) -> NonsecUserCacheableW<FwRegsBrScrm64bClk2ToScrpMiscClk2L0FwRegion8Permission0Spec> {
        NonsecUserCacheableW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Non-secure user debug allowed."]
    #[inline(always)]
    #[must_use]
    pub fn nonsec_user_debug(
        &mut self,
    ) -> NonsecUserDebugW<FwRegsBrScrm64bClk2ToScrpMiscClk2L0FwRegion8Permission0Spec> {
        NonsecUserDebugW::new(self, 15)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Allowed privid."]
    #[inline(always)]
    #[must_use]
    pub fn priv_id(
        &mut self,
    ) -> PrivIdW<FwRegsBrScrm64bClk2ToScrpMiscClk2L0FwRegion8Permission0Spec> {
        PrivIdW::new(self, 16)
    }
}
#[doc = "The FW Region 8 Permission 0 Register defines the permissions for the slave br_scrm_64b_clk2_to_scrp_misc_clk2_l0 region 8 firewall.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fw_regs_br_scrm_64b_clk2_to_scrp_misc_clk2_l0_fw_region_8_permission_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fw_regs_br_scrm_64b_clk2_to_scrp_misc_clk2_l0_fw_region_8_permission_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FwRegsBrScrm64bClk2ToScrpMiscClk2L0FwRegion8Permission0Spec;
impl crate::RegisterSpec for FwRegsBrScrm64bClk2ToScrpMiscClk2L0FwRegion8Permission0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fw_regs_br_scrm_64b_clk2_to_scrp_misc_clk2_l0_fw_region_8_permission_0::R`](R) reader structure"]
impl crate::Readable for FwRegsBrScrm64bClk2ToScrpMiscClk2L0FwRegion8Permission0Spec {}
#[doc = "`write(|w| ..)` method takes [`fw_regs_br_scrm_64b_clk2_to_scrp_misc_clk2_l0_fw_region_8_permission_0::W`](W) writer structure"]
impl crate::Writable for FwRegsBrScrm64bClk2ToScrpMiscClk2L0FwRegion8Permission0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FW_REGS_br_scrm_64b_clk2_to_scrp_misc_clk2_l0_fw_region_8_permission_0 to value 0"]
impl crate::Resettable for FwRegsBrScrm64bClk2ToScrpMiscClk2L0FwRegion8Permission0Spec {
    const RESET_VALUE: u32 = 0;
}
