#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_cq_send_sts_config1` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgCqSendStsConfig1Spec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_cq_send_sts_config1` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgCqSendStsConfig1Spec>;
#[doc = "Field `CMD_IDLE_TIMER` reader - 15:0\\]
This field indicates to CQE the polling period to use when using periodic SEND_QUEUE_STATUS \\[CMD13\\]
polling.Periodic polling is used when tasks are pending in the device, but no data transfer is in progress. When a SEND_QUEUE_STATUS response indicating that no task is ready for execution, CQE counts the configured time until it issues the next SEND_QUEUE_STATUS.Timer units are clock periods of theclock whose fre-quency is specified in the Internal Timer Clock Fre-quency field CQCAP register.The minimum value is 0001h \\[1 clock period\\]
and the maximum value is FFFFh \\[65535 clock periods\\]. Default interval is: 4096 clock periods. For example, a CQCAP field value of 0 indicates a 19.2 MHz clock frequency \\[period = 52.08 ns\\]. If the setting in CQSST is 1000h, the calcu.lated polling period is 4096*52.08 ns= 213.33 us."]
pub type CmdIdleTimerR = crate::FieldReader<u16>;
#[doc = "Field `CMD_IDLE_TIMER` writer - 15:0\\]
This field indicates to CQE the polling period to use when using periodic SEND_QUEUE_STATUS \\[CMD13\\]
polling.Periodic polling is used when tasks are pending in the device, but no data transfer is in progress. When a SEND_QUEUE_STATUS response indicating that no task is ready for execution, CQE counts the configured time until it issues the next SEND_QUEUE_STATUS.Timer units are clock periods of theclock whose fre-quency is specified in the Internal Timer Clock Fre-quency field CQCAP register.The minimum value is 0001h \\[1 clock period\\]
and the maximum value is FFFFh \\[65535 clock periods\\]. Default interval is: 4096 clock periods. For example, a CQCAP field value of 0 indicates a 19.2 MHz clock frequency \\[period = 52.08 ns\\]. If the setting in CQSST is 1000h, the calcu.lated polling period is 4096*52.08 ns= 213.33 us."]
pub type CmdIdleTimerW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CMD_BLK_CNTR` reader - 19:16\\]
This field indicates to CQE when to send SEND_QUEUE_STATUS \\[CMD13\\]
command to inquire the status of the devices task queue.A value of n means CQE shall send status command on the CMD line, during the transfer of data block BLOCK_CNT-n, on the data lines, where BLOCK_CNT is the number of blocks in the current transaction.A value of 0 means that SEND_QUEUE_STATUS \\[CMD13\\]
command shall not be sent during the transac-tion. Instead it will be sent only when the data lines are idle.A value of 1 means that STATUS command is to be sent during the last block of the transaction."]
pub type CmdBlkCntrR = crate::FieldReader;
#[doc = "Field `CMD_BLK_CNTR` writer - 19:16\\]
This field indicates to CQE when to send SEND_QUEUE_STATUS \\[CMD13\\]
command to inquire the status of the devices task queue.A value of n means CQE shall send status command on the CMD line, during the transfer of data block BLOCK_CNT-n, on the data lines, where BLOCK_CNT is the number of blocks in the current transaction.A value of 0 means that SEND_QUEUE_STATUS \\[CMD13\\]
command shall not be sent during the transac-tion. Instead it will be sent only when the data lines are idle.A value of 1 means that STATUS command is to be sent during the last block of the transaction."]
pub type CmdBlkCntrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
This field indicates to CQE the polling period to use when using periodic SEND_QUEUE_STATUS \\[CMD13\\]
polling.Periodic polling is used when tasks are pending in the device, but no data transfer is in progress. When a SEND_QUEUE_STATUS response indicating that no task is ready for execution, CQE counts the configured time until it issues the next SEND_QUEUE_STATUS.Timer units are clock periods of theclock whose fre-quency is specified in the Internal Timer Clock Fre-quency field CQCAP register.The minimum value is 0001h \\[1 clock period\\]
and the maximum value is FFFFh \\[65535 clock periods\\]. Default interval is: 4096 clock periods. For example, a CQCAP field value of 0 indicates a 19.2 MHz clock frequency \\[period = 52.08 ns\\]. If the setting in CQSST is 1000h, the calcu.lated polling period is 4096*52.08 ns= 213.33 us."]
    #[inline(always)]
    pub fn cmd_idle_timer(&self) -> CmdIdleTimerR {
        CmdIdleTimerR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - 19:16\\]
This field indicates to CQE when to send SEND_QUEUE_STATUS \\[CMD13\\]
command to inquire the status of the devices task queue.A value of n means CQE shall send status command on the CMD line, during the transfer of data block BLOCK_CNT-n, on the data lines, where BLOCK_CNT is the number of blocks in the current transaction.A value of 0 means that SEND_QUEUE_STATUS \\[CMD13\\]
command shall not be sent during the transac-tion. Instead it will be sent only when the data lines are idle.A value of 1 means that STATUS command is to be sent during the last block of the transaction."]
    #[inline(always)]
    pub fn cmd_blk_cntr(&self) -> CmdBlkCntrR {
        CmdBlkCntrR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
This field indicates to CQE the polling period to use when using periodic SEND_QUEUE_STATUS \\[CMD13\\]
polling.Periodic polling is used when tasks are pending in the device, but no data transfer is in progress. When a SEND_QUEUE_STATUS response indicating that no task is ready for execution, CQE counts the configured time until it issues the next SEND_QUEUE_STATUS.Timer units are clock periods of theclock whose fre-quency is specified in the Internal Timer Clock Fre-quency field CQCAP register.The minimum value is 0001h \\[1 clock period\\]
and the maximum value is FFFFh \\[65535 clock periods\\]. Default interval is: 4096 clock periods. For example, a CQCAP field value of 0 indicates a 19.2 MHz clock frequency \\[period = 52.08 ns\\]. If the setting in CQSST is 1000h, the calcu.lated polling period is 4096*52.08 ns= 213.33 us."]
    #[inline(always)]
    #[must_use]
    pub fn cmd_idle_timer(&mut self) -> CmdIdleTimerW<SdhcWrap_CtlCfg_CtlcfgCqSendStsConfig1Spec> {
        CmdIdleTimerW::new(self, 0)
    }
    #[doc = "Bits 16:19 - 19:16\\]
This field indicates to CQE when to send SEND_QUEUE_STATUS \\[CMD13\\]
command to inquire the status of the devices task queue.A value of n means CQE shall send status command on the CMD line, during the transfer of data block BLOCK_CNT-n, on the data lines, where BLOCK_CNT is the number of blocks in the current transaction.A value of 0 means that SEND_QUEUE_STATUS \\[CMD13\\]
command shall not be sent during the transac-tion. Instead it will be sent only when the data lines are idle.A value of 1 means that STATUS command is to be sent during the last block of the transaction."]
    #[inline(always)]
    #[must_use]
    pub fn cmd_blk_cntr(&mut self) -> CmdBlkCntrW<SdhcWrap_CtlCfg_CtlcfgCqSendStsConfig1Spec> {
        CmdBlkCntrW::new(self, 16)
    }
}
#[doc = "The register controls the when SEND_QUEUE_STATUS commands are sent.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_cq_send_sts_config1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_cq_send_sts_config1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgCqSendStsConfig1Spec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgCqSendStsConfig1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_cq_send_sts_config1::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgCqSendStsConfig1Spec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_cq_send_sts_config1::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgCqSendStsConfig1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_cq_send_sts_config1 to value 0x0001_4096"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgCqSendStsConfig1Spec {
    const RESET_VALUE: u32 = 0x0001_4096;
}
