#[doc = "Register `APBADDR_PMU_CPU0_PMCEID0_EL0` reader"]
pub type R = crate::R<ApbaddrPmuCpu0Pmceid0El0Spec>;
#[doc = "Register `APBADDR_PMU_CPU0_PMCEID0_EL0` writer"]
pub type W = crate::W<ApbaddrPmuCpu0Pmceid0El0Spec>;
#[doc = "Field `CE_31_0` reader - 31:0\\]
Common architectural and microarchitectural feature events that can be counted by the PMU event counters.For each bit described in the following table, the event is implemented if the bit is set to 1, or not implemented if the bit is set to 0.BitEvent numberEvent mnemonic310x01FL1D_CACHE_ALLOCATE300x01ECHAIN290x01DBUS_CYCLES280x01CTTBR_WRITE_RETIRED270x01BINST_SPEC260x01AMEMORY_ERROR250x019BUS_ACCESS240x018L2D_CACHE_WB230x017L2D_CACHE_REFILL220x016L2D_CACHE210x015L1D_CACHE_WB200x014L1I_CACHE190x013MEM_ACCESS180x012BR_PRED170x011CPU_CYCLES160x010BR_MIS_PRED150x00FUNALIGNED_LDST_RETIRED140x00EBR_RETURN_RETIRED130x00DBR_IMMED_RETIRED120x00CPC_WRITE_RETIRED110x00BCID_WRITE_RETIRED100x00AEXC_RETURN90x009EXC_TAKEN80x008INST_RETIRED70x007ST_RETIRED60x006LD_RETIRED50x005L1D_TLB_REFILL40x004L1D_CACHE30x003L1D_CACHE_REFILL20x002L1I_TLB_REFILL10x001L1I_CACHE_REFILL00x000SW_INCR"]
pub type Ce31_0R = crate::FieldReader<u32>;
#[doc = "Field `CE_31_0` writer - 31:0\\]
Common architectural and microarchitectural feature events that can be counted by the PMU event counters.For each bit described in the following table, the event is implemented if the bit is set to 1, or not implemented if the bit is set to 0.BitEvent numberEvent mnemonic310x01FL1D_CACHE_ALLOCATE300x01ECHAIN290x01DBUS_CYCLES280x01CTTBR_WRITE_RETIRED270x01BINST_SPEC260x01AMEMORY_ERROR250x019BUS_ACCESS240x018L2D_CACHE_WB230x017L2D_CACHE_REFILL220x016L2D_CACHE210x015L1D_CACHE_WB200x014L1I_CACHE190x013MEM_ACCESS180x012BR_PRED170x011CPU_CYCLES160x010BR_MIS_PRED150x00FUNALIGNED_LDST_RETIRED140x00EBR_RETURN_RETIRED130x00DBR_IMMED_RETIRED120x00CPC_WRITE_RETIRED110x00BCID_WRITE_RETIRED100x00AEXC_RETURN90x009EXC_TAKEN80x008INST_RETIRED70x007ST_RETIRED60x006LD_RETIRED50x005L1D_TLB_REFILL40x004L1D_CACHE30x003L1D_CACHE_REFILL20x002L1I_TLB_REFILL10x001L1I_CACHE_REFILL00x000SW_INCR"]
pub type Ce31_0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Common architectural and microarchitectural feature events that can be counted by the PMU event counters.For each bit described in the following table, the event is implemented if the bit is set to 1, or not implemented if the bit is set to 0.BitEvent numberEvent mnemonic310x01FL1D_CACHE_ALLOCATE300x01ECHAIN290x01DBUS_CYCLES280x01CTTBR_WRITE_RETIRED270x01BINST_SPEC260x01AMEMORY_ERROR250x019BUS_ACCESS240x018L2D_CACHE_WB230x017L2D_CACHE_REFILL220x016L2D_CACHE210x015L1D_CACHE_WB200x014L1I_CACHE190x013MEM_ACCESS180x012BR_PRED170x011CPU_CYCLES160x010BR_MIS_PRED150x00FUNALIGNED_LDST_RETIRED140x00EBR_RETURN_RETIRED130x00DBR_IMMED_RETIRED120x00CPC_WRITE_RETIRED110x00BCID_WRITE_RETIRED100x00AEXC_RETURN90x009EXC_TAKEN80x008INST_RETIRED70x007ST_RETIRED60x006LD_RETIRED50x005L1D_TLB_REFILL40x004L1D_CACHE30x003L1D_CACHE_REFILL20x002L1I_TLB_REFILL10x001L1I_CACHE_REFILL00x000SW_INCR"]
    #[inline(always)]
    pub fn ce_31_0(&self) -> Ce31_0R {
        Ce31_0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Common architectural and microarchitectural feature events that can be counted by the PMU event counters.For each bit described in the following table, the event is implemented if the bit is set to 1, or not implemented if the bit is set to 0.BitEvent numberEvent mnemonic310x01FL1D_CACHE_ALLOCATE300x01ECHAIN290x01DBUS_CYCLES280x01CTTBR_WRITE_RETIRED270x01BINST_SPEC260x01AMEMORY_ERROR250x019BUS_ACCESS240x018L2D_CACHE_WB230x017L2D_CACHE_REFILL220x016L2D_CACHE210x015L1D_CACHE_WB200x014L1I_CACHE190x013MEM_ACCESS180x012BR_PRED170x011CPU_CYCLES160x010BR_MIS_PRED150x00FUNALIGNED_LDST_RETIRED140x00EBR_RETURN_RETIRED130x00DBR_IMMED_RETIRED120x00CPC_WRITE_RETIRED110x00BCID_WRITE_RETIRED100x00AEXC_RETURN90x009EXC_TAKEN80x008INST_RETIRED70x007ST_RETIRED60x006LD_RETIRED50x005L1D_TLB_REFILL40x004L1D_CACHE30x003L1D_CACHE_REFILL20x002L1I_TLB_REFILL10x001L1I_CACHE_REFILL00x000SW_INCR"]
    #[inline(always)]
    #[must_use]
    pub fn ce_31_0(&mut self) -> Ce31_0W<ApbaddrPmuCpu0Pmceid0El0Spec> {
        Ce31_0W::new(self, 0)
    }
}
#[doc = "Performance Monitors Common Event Identification Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu0_pmceid0_el0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu0_pmceid0_el0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrPmuCpu0Pmceid0El0Spec;
impl crate::RegisterSpec for ApbaddrPmuCpu0Pmceid0El0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_pmu_cpu0_pmceid0_el0::R`](R) reader structure"]
impl crate::Readable for ApbaddrPmuCpu0Pmceid0El0Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_pmu_cpu0_pmceid0_el0::W`](W) writer structure"]
impl crate::Writable for ApbaddrPmuCpu0Pmceid0El0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_PMU_CPU0_PMCEID0_EL0 to value 0"]
impl crate::Resettable for ApbaddrPmuCpu0Pmceid0El0Spec {
    const RESET_VALUE: u32 = 0;
}
