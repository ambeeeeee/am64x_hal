#[doc = "Register `PR1_ICSS_ECAP0__ECAP_SLV__REGS_ECCTL2_ECCTL1` reader"]
pub type R = crate::R<Pr1IcssEcap0_EcapSlv_RegsEcctl2Ecctl1Spec>;
#[doc = "Register `PR1_ICSS_ECAP0__ECAP_SLV__REGS_ECCTL2_ECCTL1` writer"]
pub type W = crate::W<Pr1IcssEcap0_EcapSlv_RegsEcctl2Ecctl1Spec>;
#[doc = "Field `CAP1POL` reader - 0:0\\]
CAPTURE EVENT 1 POLARITY SELECT:0CAPTURE EVENT 1 TRIGGERED ON A RISING EDGE \\[FE\\]1CAPTURE EVENT 1 TRIGGERED ON A FALLING EDGE \\[FE\\]"]
pub type Cap1polR = crate::BitReader;
#[doc = "Field `CAP1POL` writer - 0:0\\]
CAPTURE EVENT 1 POLARITY SELECT:0CAPTURE EVENT 1 TRIGGERED ON A RISING EDGE \\[FE\\]1CAPTURE EVENT 1 TRIGGERED ON A FALLING EDGE \\[FE\\]"]
pub type Cap1polW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRRST1` reader - 1:1\\]
COUNTER RESET ON CAPTURE EVENT 1:0DO NOT RESET COUNTER ON CAPTURE EVENT 1 \\[ABSOLUTE TIME STAMP\\]1RESET COUNTER AFTER EVENT 1 TIME-STAMP HAS BEEN CAPTURED\\[USED IN DIFFERENCE MODE OPERATION\\]"]
pub type Ctrrst1R = crate::BitReader;
#[doc = "Field `CTRRST1` writer - 1:1\\]
COUNTER RESET ON CAPTURE EVENT 1:0DO NOT RESET COUNTER ON CAPTURE EVENT 1 \\[ABSOLUTE TIME STAMP\\]1RESET COUNTER AFTER EVENT 1 TIME-STAMP HAS BEEN CAPTURED\\[USED IN DIFFERENCE MODE OPERATION\\]"]
pub type Ctrrst1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP2POL` reader - 2:2\\]
CAPTURE EVENT 2 POLARITY SELECT:0CAPTURE EVENT 2 TRIGGERED ON A RISING EDGE \\[FE\\]1CAPTURE EVENT 2 TRIGGERED ON A FALLING EDGE \\[FE\\]"]
pub type Cap2polR = crate::BitReader;
#[doc = "Field `CAP2POL` writer - 2:2\\]
CAPTURE EVENT 2 POLARITY SELECT:0CAPTURE EVENT 2 TRIGGERED ON A RISING EDGE \\[FE\\]1CAPTURE EVENT 2 TRIGGERED ON A FALLING EDGE \\[FE\\]"]
pub type Cap2polW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRRST2` reader - 3:3\\]
COUNTER RESET ON CAPTURE EVENT 2:0DO NOT RESET COUNTER ON CAPTURE EVENT 2 \\[ABSOLUTE TIME STAMP\\]1RESET COUNTER AFTER EVENT 2 TIME-STAMP HAS BEEN CAPTURED\\[USED IN DIFFERENCE MODE OPERATION\\]"]
pub type Ctrrst2R = crate::BitReader;
#[doc = "Field `CTRRST2` writer - 3:3\\]
COUNTER RESET ON CAPTURE EVENT 2:0DO NOT RESET COUNTER ON CAPTURE EVENT 2 \\[ABSOLUTE TIME STAMP\\]1RESET COUNTER AFTER EVENT 2 TIME-STAMP HAS BEEN CAPTURED\\[USED IN DIFFERENCE MODE OPERATION\\]"]
pub type Ctrrst2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP3POL` reader - 4:4\\]
CAPTURE EVENT 3 POLARITY SELECT:0CAPTURE EVENT 3 TRIGGERED ON A RISING EDGE \\[FE\\]1CAPTURE EVENT 3 TRIGGERED ON A FALLING EDGE \\[FE\\]"]
pub type Cap3polR = crate::BitReader;
#[doc = "Field `CAP3POL` writer - 4:4\\]
CAPTURE EVENT 3 POLARITY SELECT:0CAPTURE EVENT 3 TRIGGERED ON A RISING EDGE \\[FE\\]1CAPTURE EVENT 3 TRIGGERED ON A FALLING EDGE \\[FE\\]"]
pub type Cap3polW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRRST3` reader - 5:5\\]
COUNTER RESET ON CAPTURE EVENT 3:0DO NOT RESET COUNTER ON CAPTURE EVENT 3 \\[ABSOLUTE TIME STAMP\\]1RESET COUNTER AFTER EVENT 3 TIME-STAMP HAS BEEN CAPTURED\\[USED IN DIFFERENCE MODE OPERATION\\]"]
pub type Ctrrst3R = crate::BitReader;
#[doc = "Field `CTRRST3` writer - 5:5\\]
COUNTER RESET ON CAPTURE EVENT 3:0DO NOT RESET COUNTER ON CAPTURE EVENT 3 \\[ABSOLUTE TIME STAMP\\]1RESET COUNTER AFTER EVENT 3 TIME-STAMP HAS BEEN CAPTURED\\[USED IN DIFFERENCE MODE OPERATION\\]"]
pub type Ctrrst3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP4POL` reader - 6:6\\]
CAPTURE EVENT 4 POLARITY SELECT:0CAPTURE EVENT 4 TRIGGERED ON A RISING EDGE \\[FE\\]1CAPTURE EVENT 4 TRIGGERED ON A FALLING EDGE \\[FE\\]"]
pub type Cap4polR = crate::BitReader;
#[doc = "Field `CAP4POL` writer - 6:6\\]
CAPTURE EVENT 4 POLARITY SELECT:0CAPTURE EVENT 4 TRIGGERED ON A RISING EDGE \\[FE\\]1CAPTURE EVENT 4 TRIGGERED ON A FALLING EDGE \\[FE\\]"]
pub type Cap4polW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRRST4` reader - 7:7\\]
COUNTER RESET ON CAPTURE EVENT 4:0DO NOT RESET COUNTER ON CAPTURE EVENT 4 \\[ABSOLUTE TIME STAMP\\]1RESET COUNTER AFTER EVENT 4 TIME-STAMP HAS BEEN CAPTURED\\[USED IN DIFFERENCE MODE OPERATION\\]"]
pub type Ctrrst4R = crate::BitReader;
#[doc = "Field `CTRRST4` writer - 7:7\\]
COUNTER RESET ON CAPTURE EVENT 4:0DO NOT RESET COUNTER ON CAPTURE EVENT 4 \\[ABSOLUTE TIME STAMP\\]1RESET COUNTER AFTER EVENT 4 TIME-STAMP HAS BEEN CAPTURED\\[USED IN DIFFERENCE MODE OPERATION\\]"]
pub type Ctrrst4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPLDEN` reader - 8:8\\]
ENABLE LOADING OF CAP1-4 REGISTERS ON A CAPTURE EVENT:0DISABLE CAP1-4 REGISTER LOADS AT CAPTURE EVENT TIME1ENABLE CAP1-4 REGISTER LOADS AT CAPTURE EVENT TIME"]
pub type CapldenR = crate::BitReader;
#[doc = "Field `CAPLDEN` writer - 8:8\\]
ENABLE LOADING OF CAP1-4 REGISTERS ON A CAPTURE EVENT:0DISABLE CAP1-4 REGISTER LOADS AT CAPTURE EVENT TIME1ENABLE CAP1-4 REGISTER LOADS AT CAPTURE EVENT TIME"]
pub type CapldenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVTFLTPS` reader - 13:9\\]
EVENT FILTER PRESCALE SELECT:0,0,0,0,0DIVIDE BY 1 \\[IE NO PRESCALE, BY-PASS THE PRESCALER\\]0,0,0,0,1DIVIDE BY 20,0,0,1,0DIVIDE BY 40,0,0,1,1DIVIDE BY 60,0,1,0,0DIVIDE BY 80,0,1,0,1DIVIDE BY 10 1,1,1,1,0DIVIDE BY 601,1,1,1,1DIVIDE BY 62"]
pub type EvtfltpsR = crate::FieldReader;
#[doc = "Field `EVTFLTPS` writer - 13:9\\]
EVENT FILTER PRESCALE SELECT:0,0,0,0,0DIVIDE BY 1 \\[IE NO PRESCALE, BY-PASS THE PRESCALER\\]0,0,0,0,1DIVIDE BY 20,0,0,1,0DIVIDE BY 40,0,0,1,1DIVIDE BY 60,0,1,0,0DIVIDE BY 80,0,1,0,1DIVIDE BY 10 1,1,1,1,0DIVIDE BY 601,1,1,1,1DIVIDE BY 62"]
pub type EvtfltpsW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SOFT` reader - 14:14\\]
EMULATION CONTROL0,0 TSCNT COUNTER STOPS IMMEDIATELY ON EMULATION SUSPEND0,1 TSCNT COUNTER RUNS UNTIL = 01,X TSCNT COUNTER IS UNAFFECTED BY EMULATION SUSPEND \\[RUN FREE\\]"]
pub type SoftR = crate::BitReader;
#[doc = "Field `SOFT` writer - 14:14\\]
EMULATION CONTROL0,0 TSCNT COUNTER STOPS IMMEDIATELY ON EMULATION SUSPEND0,1 TSCNT COUNTER RUNS UNTIL = 01,X TSCNT COUNTER IS UNAFFECTED BY EMULATION SUSPEND \\[RUN FREE\\]"]
pub type SoftW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FREE` reader - 15:15\\]
EMULATION CONTROL0,0 TSCNT COUNTER STOPS IMMEDIATELY ON EMULATION SUSPEND0,1 TSCNT COUNTER RUNS UNTIL = 01,X TSCNT COUNTER IS UNAFFECTED BY EMULATION SUSPEND \\[RUN FREE\\]"]
pub type FreeR = crate::BitReader;
#[doc = "Field `FREE` writer - 15:15\\]
EMULATION CONTROL0,0 TSCNT COUNTER STOPS IMMEDIATELY ON EMULATION SUSPEND0,1 TSCNT COUNTER RUNS UNTIL = 01,X TSCNT COUNTER IS UNAFFECTED BY EMULATION SUSPEND \\[RUN FREE\\]"]
pub type FreeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONT_ONESHT` reader - 16:16\\]
CONTINUOUS OR ONESHOT MODE CONTROL:\\[APPLICABLE ONLY IN CAPTURE MODE\\]0OPERATE IN CONTINUOUS MODE1OPERATE IN ONE-SHOT MODE"]
pub type ContOneshtR = crate::BitReader;
#[doc = "Field `CONT_ONESHT` writer - 16:16\\]
CONTINUOUS OR ONESHOT MODE CONTROL:\\[APPLICABLE ONLY IN CAPTURE MODE\\]0OPERATE IN CONTINUOUS MODE1OPERATE IN ONE-SHOT MODE"]
pub type ContOneshtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOPVALUE` reader - 18:17\\]
STOP VALUE FOR ONE-SHOT MODE:THIS IS THE NUMBER \\[BETWEEN 1-4\\]
OF CAPTURES ALLOWED TO OCCUR BEFORE THE CAP\\[1-4\\]
REGISTERS ARE FROZEN, IECAPTURE SEQUENCE IS STOPPED0,0STOP AFTER CAPTURE EVENT 10,1STOP AFTER CAPTURE EVENT 21,0STOP AFTER CAPTURE EVENT 31,1STOP AFTER CAPTURE EVENT 4NOTES: \\[1\\]
STOPVALUE IS COMPARED TO MOD4 COUNTER, WHEN EQUAL, 2 ACTIONS OCCUR:1\\]
MOD4 COUNTER IS STOPPED \\[FROZEN\\]2\\]
CAPTURE REGISTER LOADS ARE INHIBITED\\[2\\]
IN ONE SHOT MODE, FURTHER INTERRUPT EVENTS ARE BLOCKED UNTIL WE RE-ARM, ONCE THE NUMBER OF EVENTS CAPTURED HAS BEEN REACHED"]
pub type StopvalueR = crate::FieldReader;
#[doc = "Field `STOPVALUE` writer - 18:17\\]
STOP VALUE FOR ONE-SHOT MODE:THIS IS THE NUMBER \\[BETWEEN 1-4\\]
OF CAPTURES ALLOWED TO OCCUR BEFORE THE CAP\\[1-4\\]
REGISTERS ARE FROZEN, IECAPTURE SEQUENCE IS STOPPED0,0STOP AFTER CAPTURE EVENT 10,1STOP AFTER CAPTURE EVENT 21,0STOP AFTER CAPTURE EVENT 31,1STOP AFTER CAPTURE EVENT 4NOTES: \\[1\\]
STOPVALUE IS COMPARED TO MOD4 COUNTER, WHEN EQUAL, 2 ACTIONS OCCUR:1\\]
MOD4 COUNTER IS STOPPED \\[FROZEN\\]2\\]
CAPTURE REGISTER LOADS ARE INHIBITED\\[2\\]
IN ONE SHOT MODE, FURTHER INTERRUPT EVENTS ARE BLOCKED UNTIL WE RE-ARM, ONCE THE NUMBER OF EVENTS CAPTURED HAS BEEN REACHED"]
pub type StopvalueW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REARM_RESET` reader - 19:19\\]
ONE-SHOT RE-ARMING, IE WAIT FOR STOP TRIGGER:WRITING A ONE ARMS THE ONE-SHOT SEQUENCE, IE:1\\]
RESETS THE MOD4 COUNTER TO ZERO2\\]
UN-FREEZES THE MOD4 COUNTER3\\]
ENABLES CAPTURE REGISTER LOADSWRITING A ZERO HAS NO EFFECT ALWAYS RETURNS A 0NOTE: THE RE-ARM FUNCTION IS VALID IN ONESHT OR CONTINOUS MODE"]
pub type RearmResetR = crate::BitReader;
#[doc = "Field `REARM_RESET` writer - 19:19\\]
ONE-SHOT RE-ARMING, IE WAIT FOR STOP TRIGGER:WRITING A ONE ARMS THE ONE-SHOT SEQUENCE, IE:1\\]
RESETS THE MOD4 COUNTER TO ZERO2\\]
UN-FREEZES THE MOD4 COUNTER3\\]
ENABLES CAPTURE REGISTER LOADSWRITING A ZERO HAS NO EFFECT ALWAYS RETURNS A 0NOTE: THE RE-ARM FUNCTION IS VALID IN ONESHT OR CONTINOUS MODE"]
pub type RearmResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSCNTSTP` reader - 20:20\\]
COUNTER STOP \\[FREEZE\\]
CONTROL:0COUNTER STOPPED1COUNTER FREE RUNNING"]
pub type TscntstpR = crate::BitReader;
#[doc = "Field `TSCNTSTP` writer - 20:20\\]
COUNTER STOP \\[FREEZE\\]
CONTROL:0COUNTER STOPPED1COUNTER FREE RUNNING"]
pub type TscntstpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCI_EN` reader - 21:21\\]
COUNTER \\[TSCNT\\]
SYNC-IN SELECT MODE:0DISABLE SYNC-IN OPTION1ENABLE COUNTER \\[TSCNT\\]
TO BE LOADED FROM CNTPHS REGISTER UPON EITHER A SYNCI SIGNAL OR A S/W FORCE EVENT"]
pub type SynciEnR = crate::BitReader;
#[doc = "Field `SYNCI_EN` writer - 21:21\\]
COUNTER \\[TSCNT\\]
SYNC-IN SELECT MODE:0DISABLE SYNC-IN OPTION1ENABLE COUNTER \\[TSCNT\\]
TO BE LOADED FROM CNTPHS REGISTER UPON EITHER A SYNCI SIGNAL OR A S/W FORCE EVENT"]
pub type SynciEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCO_SEL` reader - 23:22\\]
SYNC-OUT SELECT:0,0SELECT SYNC-IN EVENT TO BE THE SYNC-OUT SIGNAL \\[PASS THROUGH\\]0,1SELECT PRD_EQ EVENT TO BE THE SYNC-OUT SIGNAL1,0DISABLE SYNC OUT SIGNAL1,1DISABLE SYNC OUT SIGNALNOTE: SELECTION PRD_EQ IS MEANINGFUL ONLY IN APWM MODE, HOWEVER CAN STILL BE CHOSEN IN CAP MODE IF USER BELIEVES IT TO BE USEFUL"]
pub type SyncoSelR = crate::FieldReader;
#[doc = "Field `SYNCO_SEL` writer - 23:22\\]
SYNC-OUT SELECT:0,0SELECT SYNC-IN EVENT TO BE THE SYNC-OUT SIGNAL \\[PASS THROUGH\\]0,1SELECT PRD_EQ EVENT TO BE THE SYNC-OUT SIGNAL1,0DISABLE SYNC OUT SIGNAL1,1DISABLE SYNC OUT SIGNALNOTE: SELECTION PRD_EQ IS MEANINGFUL ONLY IN APWM MODE, HOWEVER CAN STILL BE CHOSEN IN CAP MODE IF USER BELIEVES IT TO BE USEFUL"]
pub type SyncoSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SWSYNC` reader - 24:24\\]
SOFTWARE FORCED COUNTER \\[TSCNT\\]
SYNCING:0WRITING A ZERO HAS NO EFFECT WILL ALWAYS RETURN A ZERO1WRITING A ONE WILL FORCE A TSCNT SHADOW LOAD OF CURRENT ECAP MODULE AND ANY ECAP MODULES DOWN-STREAM PROVIDING THE SYNCO_SEL BITS ARE 0,0 AFTER WRITING A ONE THIS BIT RETURNS TO A ZERONOTE: THIS PROVIDES A CONVENIENT S/W METHOD TO SYNCHRONIZE SOME OR ALL ECAP TIMEBASES IN APWM MODE THE SYNCING CAN ALSO BE DONE VIA THE PRD_EQ EVENT"]
pub type SwsyncR = crate::BitReader;
#[doc = "Field `SWSYNC` writer - 24:24\\]
SOFTWARE FORCED COUNTER \\[TSCNT\\]
SYNCING:0WRITING A ZERO HAS NO EFFECT WILL ALWAYS RETURN A ZERO1WRITING A ONE WILL FORCE A TSCNT SHADOW LOAD OF CURRENT ECAP MODULE AND ANY ECAP MODULES DOWN-STREAM PROVIDING THE SYNCO_SEL BITS ARE 0,0 AFTER WRITING A ONE THIS BIT RETURNS TO A ZERONOTE: THIS PROVIDES A CONVENIENT S/W METHOD TO SYNCHRONIZE SOME OR ALL ECAP TIMEBASES IN APWM MODE THE SYNCING CAN ALSO BE DONE VIA THE PRD_EQ EVENT"]
pub type SwsyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP_APWM` reader - 25:25\\]
CAP/APWM OPERATING MODE SELECT:0ECAP MODULE OPERATES IN CAPTURE MODETHIS MODE FORCES THE FOLLOWING CONFIGURATION:1\\]
INHIBITS TSCNT RESETS VIA PRD_EQ EVENT2\\]
INHIBITS SHADOW LOADS ON CAP1 &amp;AMP"]
pub type CapApwmR = crate::BitReader;
#[doc = "Field `CAP_APWM` writer - 25:25\\]
CAP/APWM OPERATING MODE SELECT:0ECAP MODULE OPERATES IN CAPTURE MODETHIS MODE FORCES THE FOLLOWING CONFIGURATION:1\\]
INHIBITS TSCNT RESETS VIA PRD_EQ EVENT2\\]
INHIBITS SHADOW LOADS ON CAP1 &amp;AMP"]
pub type CapApwmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APWMPOL` reader - 26:26\\]
APWM OUTPUT POLARITY SELECT:0OUTPUT IS ACTIVE HIGH \\[IE COMPARE VALUE DEFINES HIGH TIME\\]1OUTPUT IS ACTIVE LOW \\[IE COMPARE VALUE DEFINES LOW TIME\\]NOTE: THIS IS APPLICABLE ONLY IN APWM OPERATING MODE"]
pub type ApwmpolR = crate::BitReader;
#[doc = "Field `APWMPOL` writer - 26:26\\]
APWM OUTPUT POLARITY SELECT:0OUTPUT IS ACTIVE HIGH \\[IE COMPARE VALUE DEFINES HIGH TIME\\]1OUTPUT IS ACTIVE LOW \\[IE COMPARE VALUE DEFINES LOW TIME\\]NOTE: THIS IS APPLICABLE ONLY IN APWM OPERATING MODE"]
pub type ApwmpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FILTER` reader - "]
pub type FilterR = crate::FieldReader;
#[doc = "Field `FILTER` writer - "]
pub type FilterW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
CAPTURE EVENT 1 POLARITY SELECT:0CAPTURE EVENT 1 TRIGGERED ON A RISING EDGE \\[FE\\]1CAPTURE EVENT 1 TRIGGERED ON A FALLING EDGE \\[FE\\]"]
    #[inline(always)]
    pub fn cap1pol(&self) -> Cap1polR {
        Cap1polR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
COUNTER RESET ON CAPTURE EVENT 1:0DO NOT RESET COUNTER ON CAPTURE EVENT 1 \\[ABSOLUTE TIME STAMP\\]1RESET COUNTER AFTER EVENT 1 TIME-STAMP HAS BEEN CAPTURED\\[USED IN DIFFERENCE MODE OPERATION\\]"]
    #[inline(always)]
    pub fn ctrrst1(&self) -> Ctrrst1R {
        Ctrrst1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
CAPTURE EVENT 2 POLARITY SELECT:0CAPTURE EVENT 2 TRIGGERED ON A RISING EDGE \\[FE\\]1CAPTURE EVENT 2 TRIGGERED ON A FALLING EDGE \\[FE\\]"]
    #[inline(always)]
    pub fn cap2pol(&self) -> Cap2polR {
        Cap2polR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
COUNTER RESET ON CAPTURE EVENT 2:0DO NOT RESET COUNTER ON CAPTURE EVENT 2 \\[ABSOLUTE TIME STAMP\\]1RESET COUNTER AFTER EVENT 2 TIME-STAMP HAS BEEN CAPTURED\\[USED IN DIFFERENCE MODE OPERATION\\]"]
    #[inline(always)]
    pub fn ctrrst2(&self) -> Ctrrst2R {
        Ctrrst2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
CAPTURE EVENT 3 POLARITY SELECT:0CAPTURE EVENT 3 TRIGGERED ON A RISING EDGE \\[FE\\]1CAPTURE EVENT 3 TRIGGERED ON A FALLING EDGE \\[FE\\]"]
    #[inline(always)]
    pub fn cap3pol(&self) -> Cap3polR {
        Cap3polR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
COUNTER RESET ON CAPTURE EVENT 3:0DO NOT RESET COUNTER ON CAPTURE EVENT 3 \\[ABSOLUTE TIME STAMP\\]1RESET COUNTER AFTER EVENT 3 TIME-STAMP HAS BEEN CAPTURED\\[USED IN DIFFERENCE MODE OPERATION\\]"]
    #[inline(always)]
    pub fn ctrrst3(&self) -> Ctrrst3R {
        Ctrrst3R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
CAPTURE EVENT 4 POLARITY SELECT:0CAPTURE EVENT 4 TRIGGERED ON A RISING EDGE \\[FE\\]1CAPTURE EVENT 4 TRIGGERED ON A FALLING EDGE \\[FE\\]"]
    #[inline(always)]
    pub fn cap4pol(&self) -> Cap4polR {
        Cap4polR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
COUNTER RESET ON CAPTURE EVENT 4:0DO NOT RESET COUNTER ON CAPTURE EVENT 4 \\[ABSOLUTE TIME STAMP\\]1RESET COUNTER AFTER EVENT 4 TIME-STAMP HAS BEEN CAPTURED\\[USED IN DIFFERENCE MODE OPERATION\\]"]
    #[inline(always)]
    pub fn ctrrst4(&self) -> Ctrrst4R {
        Ctrrst4R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
ENABLE LOADING OF CAP1-4 REGISTERS ON A CAPTURE EVENT:0DISABLE CAP1-4 REGISTER LOADS AT CAPTURE EVENT TIME1ENABLE CAP1-4 REGISTER LOADS AT CAPTURE EVENT TIME"]
    #[inline(always)]
    pub fn caplden(&self) -> CapldenR {
        CapldenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:13 - 13:9\\]
EVENT FILTER PRESCALE SELECT:0,0,0,0,0DIVIDE BY 1 \\[IE NO PRESCALE, BY-PASS THE PRESCALER\\]0,0,0,0,1DIVIDE BY 20,0,0,1,0DIVIDE BY 40,0,0,1,1DIVIDE BY 60,0,1,0,0DIVIDE BY 80,0,1,0,1DIVIDE BY 10 1,1,1,1,0DIVIDE BY 601,1,1,1,1DIVIDE BY 62"]
    #[inline(always)]
    pub fn evtfltps(&self) -> EvtfltpsR {
        EvtfltpsR::new(((self.bits >> 9) & 0x1f) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
EMULATION CONTROL0,0 TSCNT COUNTER STOPS IMMEDIATELY ON EMULATION SUSPEND0,1 TSCNT COUNTER RUNS UNTIL = 01,X TSCNT COUNTER IS UNAFFECTED BY EMULATION SUSPEND \\[RUN FREE\\]"]
    #[inline(always)]
    pub fn soft(&self) -> SoftR {
        SoftR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
EMULATION CONTROL0,0 TSCNT COUNTER STOPS IMMEDIATELY ON EMULATION SUSPEND0,1 TSCNT COUNTER RUNS UNTIL = 01,X TSCNT COUNTER IS UNAFFECTED BY EMULATION SUSPEND \\[RUN FREE\\]"]
    #[inline(always)]
    pub fn free(&self) -> FreeR {
        FreeR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
CONTINUOUS OR ONESHOT MODE CONTROL:\\[APPLICABLE ONLY IN CAPTURE MODE\\]0OPERATE IN CONTINUOUS MODE1OPERATE IN ONE-SHOT MODE"]
    #[inline(always)]
    pub fn cont_onesht(&self) -> ContOneshtR {
        ContOneshtR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - 18:17\\]
STOP VALUE FOR ONE-SHOT MODE:THIS IS THE NUMBER \\[BETWEEN 1-4\\]
OF CAPTURES ALLOWED TO OCCUR BEFORE THE CAP\\[1-4\\]
REGISTERS ARE FROZEN, IECAPTURE SEQUENCE IS STOPPED0,0STOP AFTER CAPTURE EVENT 10,1STOP AFTER CAPTURE EVENT 21,0STOP AFTER CAPTURE EVENT 31,1STOP AFTER CAPTURE EVENT 4NOTES: \\[1\\]
STOPVALUE IS COMPARED TO MOD4 COUNTER, WHEN EQUAL, 2 ACTIONS OCCUR:1\\]
MOD4 COUNTER IS STOPPED \\[FROZEN\\]2\\]
CAPTURE REGISTER LOADS ARE INHIBITED\\[2\\]
IN ONE SHOT MODE, FURTHER INTERRUPT EVENTS ARE BLOCKED UNTIL WE RE-ARM, ONCE THE NUMBER OF EVENTS CAPTURED HAS BEEN REACHED"]
    #[inline(always)]
    pub fn stopvalue(&self) -> StopvalueR {
        StopvalueR::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - 19:19\\]
ONE-SHOT RE-ARMING, IE WAIT FOR STOP TRIGGER:WRITING A ONE ARMS THE ONE-SHOT SEQUENCE, IE:1\\]
RESETS THE MOD4 COUNTER TO ZERO2\\]
UN-FREEZES THE MOD4 COUNTER3\\]
ENABLES CAPTURE REGISTER LOADSWRITING A ZERO HAS NO EFFECT ALWAYS RETURNS A 0NOTE: THE RE-ARM FUNCTION IS VALID IN ONESHT OR CONTINOUS MODE"]
    #[inline(always)]
    pub fn rearm_reset(&self) -> RearmResetR {
        RearmResetR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
COUNTER STOP \\[FREEZE\\]
CONTROL:0COUNTER STOPPED1COUNTER FREE RUNNING"]
    #[inline(always)]
    pub fn tscntstp(&self) -> TscntstpR {
        TscntstpR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
COUNTER \\[TSCNT\\]
SYNC-IN SELECT MODE:0DISABLE SYNC-IN OPTION1ENABLE COUNTER \\[TSCNT\\]
TO BE LOADED FROM CNTPHS REGISTER UPON EITHER A SYNCI SIGNAL OR A S/W FORCE EVENT"]
    #[inline(always)]
    pub fn synci_en(&self) -> SynciEnR {
        SynciEnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - 23:22\\]
SYNC-OUT SELECT:0,0SELECT SYNC-IN EVENT TO BE THE SYNC-OUT SIGNAL \\[PASS THROUGH\\]0,1SELECT PRD_EQ EVENT TO BE THE SYNC-OUT SIGNAL1,0DISABLE SYNC OUT SIGNAL1,1DISABLE SYNC OUT SIGNALNOTE: SELECTION PRD_EQ IS MEANINGFUL ONLY IN APWM MODE, HOWEVER CAN STILL BE CHOSEN IN CAP MODE IF USER BELIEVES IT TO BE USEFUL"]
    #[inline(always)]
    pub fn synco_sel(&self) -> SyncoSelR {
        SyncoSelR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
SOFTWARE FORCED COUNTER \\[TSCNT\\]
SYNCING:0WRITING A ZERO HAS NO EFFECT WILL ALWAYS RETURN A ZERO1WRITING A ONE WILL FORCE A TSCNT SHADOW LOAD OF CURRENT ECAP MODULE AND ANY ECAP MODULES DOWN-STREAM PROVIDING THE SYNCO_SEL BITS ARE 0,0 AFTER WRITING A ONE THIS BIT RETURNS TO A ZERONOTE: THIS PROVIDES A CONVENIENT S/W METHOD TO SYNCHRONIZE SOME OR ALL ECAP TIMEBASES IN APWM MODE THE SYNCING CAN ALSO BE DONE VIA THE PRD_EQ EVENT"]
    #[inline(always)]
    pub fn swsync(&self) -> SwsyncR {
        SwsyncR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
CAP/APWM OPERATING MODE SELECT:0ECAP MODULE OPERATES IN CAPTURE MODETHIS MODE FORCES THE FOLLOWING CONFIGURATION:1\\]
INHIBITS TSCNT RESETS VIA PRD_EQ EVENT2\\]
INHIBITS SHADOW LOADS ON CAP1 &amp;AMP"]
    #[inline(always)]
    pub fn cap_apwm(&self) -> CapApwmR {
        CapApwmR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
APWM OUTPUT POLARITY SELECT:0OUTPUT IS ACTIVE HIGH \\[IE COMPARE VALUE DEFINES HIGH TIME\\]1OUTPUT IS ACTIVE LOW \\[IE COMPARE VALUE DEFINES LOW TIME\\]NOTE: THIS IS APPLICABLE ONLY IN APWM OPERATING MODE"]
    #[inline(always)]
    pub fn apwmpol(&self) -> ApwmpolR {
        ApwmpolR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:31"]
    #[inline(always)]
    pub fn filter(&self) -> FilterR {
        FilterR::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
CAPTURE EVENT 1 POLARITY SELECT:0CAPTURE EVENT 1 TRIGGERED ON A RISING EDGE \\[FE\\]1CAPTURE EVENT 1 TRIGGERED ON A FALLING EDGE \\[FE\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cap1pol(&mut self) -> Cap1polW<Pr1IcssEcap0_EcapSlv_RegsEcctl2Ecctl1Spec> {
        Cap1polW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
COUNTER RESET ON CAPTURE EVENT 1:0DO NOT RESET COUNTER ON CAPTURE EVENT 1 \\[ABSOLUTE TIME STAMP\\]1RESET COUNTER AFTER EVENT 1 TIME-STAMP HAS BEEN CAPTURED\\[USED IN DIFFERENCE MODE OPERATION\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ctrrst1(&mut self) -> Ctrrst1W<Pr1IcssEcap0_EcapSlv_RegsEcctl2Ecctl1Spec> {
        Ctrrst1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
CAPTURE EVENT 2 POLARITY SELECT:0CAPTURE EVENT 2 TRIGGERED ON A RISING EDGE \\[FE\\]1CAPTURE EVENT 2 TRIGGERED ON A FALLING EDGE \\[FE\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cap2pol(&mut self) -> Cap2polW<Pr1IcssEcap0_EcapSlv_RegsEcctl2Ecctl1Spec> {
        Cap2polW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
COUNTER RESET ON CAPTURE EVENT 2:0DO NOT RESET COUNTER ON CAPTURE EVENT 2 \\[ABSOLUTE TIME STAMP\\]1RESET COUNTER AFTER EVENT 2 TIME-STAMP HAS BEEN CAPTURED\\[USED IN DIFFERENCE MODE OPERATION\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ctrrst2(&mut self) -> Ctrrst2W<Pr1IcssEcap0_EcapSlv_RegsEcctl2Ecctl1Spec> {
        Ctrrst2W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
CAPTURE EVENT 3 POLARITY SELECT:0CAPTURE EVENT 3 TRIGGERED ON A RISING EDGE \\[FE\\]1CAPTURE EVENT 3 TRIGGERED ON A FALLING EDGE \\[FE\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cap3pol(&mut self) -> Cap3polW<Pr1IcssEcap0_EcapSlv_RegsEcctl2Ecctl1Spec> {
        Cap3polW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
COUNTER RESET ON CAPTURE EVENT 3:0DO NOT RESET COUNTER ON CAPTURE EVENT 3 \\[ABSOLUTE TIME STAMP\\]1RESET COUNTER AFTER EVENT 3 TIME-STAMP HAS BEEN CAPTURED\\[USED IN DIFFERENCE MODE OPERATION\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ctrrst3(&mut self) -> Ctrrst3W<Pr1IcssEcap0_EcapSlv_RegsEcctl2Ecctl1Spec> {
        Ctrrst3W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
CAPTURE EVENT 4 POLARITY SELECT:0CAPTURE EVENT 4 TRIGGERED ON A RISING EDGE \\[FE\\]1CAPTURE EVENT 4 TRIGGERED ON A FALLING EDGE \\[FE\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cap4pol(&mut self) -> Cap4polW<Pr1IcssEcap0_EcapSlv_RegsEcctl2Ecctl1Spec> {
        Cap4polW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
COUNTER RESET ON CAPTURE EVENT 4:0DO NOT RESET COUNTER ON CAPTURE EVENT 4 \\[ABSOLUTE TIME STAMP\\]1RESET COUNTER AFTER EVENT 4 TIME-STAMP HAS BEEN CAPTURED\\[USED IN DIFFERENCE MODE OPERATION\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ctrrst4(&mut self) -> Ctrrst4W<Pr1IcssEcap0_EcapSlv_RegsEcctl2Ecctl1Spec> {
        Ctrrst4W::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
ENABLE LOADING OF CAP1-4 REGISTERS ON A CAPTURE EVENT:0DISABLE CAP1-4 REGISTER LOADS AT CAPTURE EVENT TIME1ENABLE CAP1-4 REGISTER LOADS AT CAPTURE EVENT TIME"]
    #[inline(always)]
    #[must_use]
    pub fn caplden(&mut self) -> CapldenW<Pr1IcssEcap0_EcapSlv_RegsEcctl2Ecctl1Spec> {
        CapldenW::new(self, 8)
    }
    #[doc = "Bits 9:13 - 13:9\\]
EVENT FILTER PRESCALE SELECT:0,0,0,0,0DIVIDE BY 1 \\[IE NO PRESCALE, BY-PASS THE PRESCALER\\]0,0,0,0,1DIVIDE BY 20,0,0,1,0DIVIDE BY 40,0,0,1,1DIVIDE BY 60,0,1,0,0DIVIDE BY 80,0,1,0,1DIVIDE BY 10 1,1,1,1,0DIVIDE BY 601,1,1,1,1DIVIDE BY 62"]
    #[inline(always)]
    #[must_use]
    pub fn evtfltps(&mut self) -> EvtfltpsW<Pr1IcssEcap0_EcapSlv_RegsEcctl2Ecctl1Spec> {
        EvtfltpsW::new(self, 9)
    }
    #[doc = "Bit 14 - 14:14\\]
EMULATION CONTROL0,0 TSCNT COUNTER STOPS IMMEDIATELY ON EMULATION SUSPEND0,1 TSCNT COUNTER RUNS UNTIL = 01,X TSCNT COUNTER IS UNAFFECTED BY EMULATION SUSPEND \\[RUN FREE\\]"]
    #[inline(always)]
    #[must_use]
    pub fn soft(&mut self) -> SoftW<Pr1IcssEcap0_EcapSlv_RegsEcctl2Ecctl1Spec> {
        SoftW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
EMULATION CONTROL0,0 TSCNT COUNTER STOPS IMMEDIATELY ON EMULATION SUSPEND0,1 TSCNT COUNTER RUNS UNTIL = 01,X TSCNT COUNTER IS UNAFFECTED BY EMULATION SUSPEND \\[RUN FREE\\]"]
    #[inline(always)]
    #[must_use]
    pub fn free(&mut self) -> FreeW<Pr1IcssEcap0_EcapSlv_RegsEcctl2Ecctl1Spec> {
        FreeW::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
CONTINUOUS OR ONESHOT MODE CONTROL:\\[APPLICABLE ONLY IN CAPTURE MODE\\]0OPERATE IN CONTINUOUS MODE1OPERATE IN ONE-SHOT MODE"]
    #[inline(always)]
    #[must_use]
    pub fn cont_onesht(&mut self) -> ContOneshtW<Pr1IcssEcap0_EcapSlv_RegsEcctl2Ecctl1Spec> {
        ContOneshtW::new(self, 16)
    }
    #[doc = "Bits 17:18 - 18:17\\]
STOP VALUE FOR ONE-SHOT MODE:THIS IS THE NUMBER \\[BETWEEN 1-4\\]
OF CAPTURES ALLOWED TO OCCUR BEFORE THE CAP\\[1-4\\]
REGISTERS ARE FROZEN, IECAPTURE SEQUENCE IS STOPPED0,0STOP AFTER CAPTURE EVENT 10,1STOP AFTER CAPTURE EVENT 21,0STOP AFTER CAPTURE EVENT 31,1STOP AFTER CAPTURE EVENT 4NOTES: \\[1\\]
STOPVALUE IS COMPARED TO MOD4 COUNTER, WHEN EQUAL, 2 ACTIONS OCCUR:1\\]
MOD4 COUNTER IS STOPPED \\[FROZEN\\]2\\]
CAPTURE REGISTER LOADS ARE INHIBITED\\[2\\]
IN ONE SHOT MODE, FURTHER INTERRUPT EVENTS ARE BLOCKED UNTIL WE RE-ARM, ONCE THE NUMBER OF EVENTS CAPTURED HAS BEEN REACHED"]
    #[inline(always)]
    #[must_use]
    pub fn stopvalue(&mut self) -> StopvalueW<Pr1IcssEcap0_EcapSlv_RegsEcctl2Ecctl1Spec> {
        StopvalueW::new(self, 17)
    }
    #[doc = "Bit 19 - 19:19\\]
ONE-SHOT RE-ARMING, IE WAIT FOR STOP TRIGGER:WRITING A ONE ARMS THE ONE-SHOT SEQUENCE, IE:1\\]
RESETS THE MOD4 COUNTER TO ZERO2\\]
UN-FREEZES THE MOD4 COUNTER3\\]
ENABLES CAPTURE REGISTER LOADSWRITING A ZERO HAS NO EFFECT ALWAYS RETURNS A 0NOTE: THE RE-ARM FUNCTION IS VALID IN ONESHT OR CONTINOUS MODE"]
    #[inline(always)]
    #[must_use]
    pub fn rearm_reset(&mut self) -> RearmResetW<Pr1IcssEcap0_EcapSlv_RegsEcctl2Ecctl1Spec> {
        RearmResetW::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
COUNTER STOP \\[FREEZE\\]
CONTROL:0COUNTER STOPPED1COUNTER FREE RUNNING"]
    #[inline(always)]
    #[must_use]
    pub fn tscntstp(&mut self) -> TscntstpW<Pr1IcssEcap0_EcapSlv_RegsEcctl2Ecctl1Spec> {
        TscntstpW::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
COUNTER \\[TSCNT\\]
SYNC-IN SELECT MODE:0DISABLE SYNC-IN OPTION1ENABLE COUNTER \\[TSCNT\\]
TO BE LOADED FROM CNTPHS REGISTER UPON EITHER A SYNCI SIGNAL OR A S/W FORCE EVENT"]
    #[inline(always)]
    #[must_use]
    pub fn synci_en(&mut self) -> SynciEnW<Pr1IcssEcap0_EcapSlv_RegsEcctl2Ecctl1Spec> {
        SynciEnW::new(self, 21)
    }
    #[doc = "Bits 22:23 - 23:22\\]
SYNC-OUT SELECT:0,0SELECT SYNC-IN EVENT TO BE THE SYNC-OUT SIGNAL \\[PASS THROUGH\\]0,1SELECT PRD_EQ EVENT TO BE THE SYNC-OUT SIGNAL1,0DISABLE SYNC OUT SIGNAL1,1DISABLE SYNC OUT SIGNALNOTE: SELECTION PRD_EQ IS MEANINGFUL ONLY IN APWM MODE, HOWEVER CAN STILL BE CHOSEN IN CAP MODE IF USER BELIEVES IT TO BE USEFUL"]
    #[inline(always)]
    #[must_use]
    pub fn synco_sel(&mut self) -> SyncoSelW<Pr1IcssEcap0_EcapSlv_RegsEcctl2Ecctl1Spec> {
        SyncoSelW::new(self, 22)
    }
    #[doc = "Bit 24 - 24:24\\]
SOFTWARE FORCED COUNTER \\[TSCNT\\]
SYNCING:0WRITING A ZERO HAS NO EFFECT WILL ALWAYS RETURN A ZERO1WRITING A ONE WILL FORCE A TSCNT SHADOW LOAD OF CURRENT ECAP MODULE AND ANY ECAP MODULES DOWN-STREAM PROVIDING THE SYNCO_SEL BITS ARE 0,0 AFTER WRITING A ONE THIS BIT RETURNS TO A ZERONOTE: THIS PROVIDES A CONVENIENT S/W METHOD TO SYNCHRONIZE SOME OR ALL ECAP TIMEBASES IN APWM MODE THE SYNCING CAN ALSO BE DONE VIA THE PRD_EQ EVENT"]
    #[inline(always)]
    #[must_use]
    pub fn swsync(&mut self) -> SwsyncW<Pr1IcssEcap0_EcapSlv_RegsEcctl2Ecctl1Spec> {
        SwsyncW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
CAP/APWM OPERATING MODE SELECT:0ECAP MODULE OPERATES IN CAPTURE MODETHIS MODE FORCES THE FOLLOWING CONFIGURATION:1\\]
INHIBITS TSCNT RESETS VIA PRD_EQ EVENT2\\]
INHIBITS SHADOW LOADS ON CAP1 &amp;AMP"]
    #[inline(always)]
    #[must_use]
    pub fn cap_apwm(&mut self) -> CapApwmW<Pr1IcssEcap0_EcapSlv_RegsEcctl2Ecctl1Spec> {
        CapApwmW::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
APWM OUTPUT POLARITY SELECT:0OUTPUT IS ACTIVE HIGH \\[IE COMPARE VALUE DEFINES HIGH TIME\\]1OUTPUT IS ACTIVE LOW \\[IE COMPARE VALUE DEFINES LOW TIME\\]NOTE: THIS IS APPLICABLE ONLY IN APWM OPERATING MODE"]
    #[inline(always)]
    #[must_use]
    pub fn apwmpol(&mut self) -> ApwmpolW<Pr1IcssEcap0_EcapSlv_RegsEcctl2Ecctl1Spec> {
        ApwmpolW::new(self, 26)
    }
    #[doc = "Bits 27:31"]
    #[inline(always)]
    #[must_use]
    pub fn filter(&mut self) -> FilterW<Pr1IcssEcap0_EcapSlv_RegsEcctl2Ecctl1Spec> {
        FilterW::new(self, 27)
    }
}
#[doc = "ECAP CONTROL REGISTER 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_ecap0__ecap_slv__regs_ecctl2_ecctl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_ecap0__ecap_slv__regs_ecctl2_ecctl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssEcap0_EcapSlv_RegsEcctl2Ecctl1Spec;
impl crate::RegisterSpec for Pr1IcssEcap0_EcapSlv_RegsEcctl2Ecctl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_ecap0__ecap_slv__regs_ecctl2_ecctl1::R`](R) reader structure"]
impl crate::Readable for Pr1IcssEcap0_EcapSlv_RegsEcctl2Ecctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_ecap0__ecap_slv__regs_ecctl2_ecctl1::W`](W) writer structure"]
impl crate::Writable for Pr1IcssEcap0_EcapSlv_RegsEcctl2Ecctl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_ECAP0__ECAP_SLV__REGS_ECCTL2_ECCTL1 to value 0x0006_0000"]
impl crate::Resettable for Pr1IcssEcap0_EcapSlv_RegsEcctl2Ecctl1Spec {
    const RESET_VALUE: u32 = 0x0006_0000;
}
