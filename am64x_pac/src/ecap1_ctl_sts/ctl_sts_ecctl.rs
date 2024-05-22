#[doc = "Register `CTL_STS_ECCTL` reader"]
pub type R = crate::R<CtlStsEcctlSpec>;
#[doc = "Register `CTL_STS_ECCTL` writer"]
pub type W = crate::W<CtlStsEcctlSpec>;
#[doc = "Field `CAP1POL` reader - 0:0\\]
Capture Event 1 Polarity select: 1'b0 Capture event 1 triggered on a Rising Edge (FE); 1'b1 Capture event 1 triggered on a Falling Edge (FE)"]
pub type Cap1polR = crate::BitReader;
#[doc = "Field `CAP1POL` writer - 0:0\\]
Capture Event 1 Polarity select: 1'b0 Capture event 1 triggered on a Rising Edge (FE); 1'b1 Capture event 1 triggered on a Falling Edge (FE)"]
pub type Cap1polW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRRST1` reader - 1:1\\]
Counter Reset on Capture Event 1: 1'b0 Do Not reset Counter on Capture Event 1 (absolute time stamp); 1'b1 Reset Counter after Event 1 time-stamp has been captured (used in Difference mode operation)"]
pub type Ctrrst1R = crate::BitReader;
#[doc = "Field `CTRRST1` writer - 1:1\\]
Counter Reset on Capture Event 1: 1'b0 Do Not reset Counter on Capture Event 1 (absolute time stamp); 1'b1 Reset Counter after Event 1 time-stamp has been captured (used in Difference mode operation)"]
pub type Ctrrst1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP2POL` reader - 2:2\\]
Capture Event 2 Polarity select: 1'b0 Capture event 2 triggered on a Rising Edge (FE); 1'b1 Capture event 2 triggered on a Falling Edge (FE)"]
pub type Cap2polR = crate::BitReader;
#[doc = "Field `CAP2POL` writer - 2:2\\]
Capture Event 2 Polarity select: 1'b0 Capture event 2 triggered on a Rising Edge (FE); 1'b1 Capture event 2 triggered on a Falling Edge (FE)"]
pub type Cap2polW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRRST2` reader - 3:3\\]
Counter Reset on Capture Event 2: 1'b0 Do Not reset Counter on Capture Event 2 (absolute time stamp); 1'b1 Reset Counter after Event 2 time-stamp has been captured (used in Difference mode operation)"]
pub type Ctrrst2R = crate::BitReader;
#[doc = "Field `CTRRST2` writer - 3:3\\]
Counter Reset on Capture Event 2: 1'b0 Do Not reset Counter on Capture Event 2 (absolute time stamp); 1'b1 Reset Counter after Event 2 time-stamp has been captured (used in Difference mode operation)"]
pub type Ctrrst2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP3POL` reader - 4:4\\]
Capture Event 3 Polarity select: 1'b0 Capture event 3 triggered on a Rising Edge (FE); 1'b1 Capture event 3 triggered on a Falling Edge (FE)"]
pub type Cap3polR = crate::BitReader;
#[doc = "Field `CAP3POL` writer - 4:4\\]
Capture Event 3 Polarity select: 1'b0 Capture event 3 triggered on a Rising Edge (FE); 1'b1 Capture event 3 triggered on a Falling Edge (FE)"]
pub type Cap3polW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRRST3` reader - 5:5\\]
Counter Reset on Capture Event 3: 1'b0 Do Not reset Counter on Capture Event 3 (absolute time stamp); 1'b1 Reset Counter after Event 3 time-stamp has been captured (used in Difference mode operation)"]
pub type Ctrrst3R = crate::BitReader;
#[doc = "Field `CTRRST3` writer - 5:5\\]
Counter Reset on Capture Event 3: 1'b0 Do Not reset Counter on Capture Event 3 (absolute time stamp); 1'b1 Reset Counter after Event 3 time-stamp has been captured (used in Difference mode operation)"]
pub type Ctrrst3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP4POL` reader - 6:6\\]
Capture Event 4 Polarity select: 1'b0 Capture event 4 triggered on a Rising Edge (FE); 1'b1 Capture event 4 triggered on a Falling Edge (FE)"]
pub type Cap4polR = crate::BitReader;
#[doc = "Field `CAP4POL` writer - 6:6\\]
Capture Event 4 Polarity select: 1'b0 Capture event 4 triggered on a Rising Edge (FE); 1'b1 Capture event 4 triggered on a Falling Edge (FE)"]
pub type Cap4polW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRRST4` reader - 7:7\\]
Counter Reset on Capture Event 4: 1'b0 Do Not reset Counter on Capture Event 4 (absolute time stamp); 1'b1 Reset Counter after Event 4 time-stamp has been captured (used in Difference mode operation)"]
pub type Ctrrst4R = crate::BitReader;
#[doc = "Field `CTRRST4` writer - 7:7\\]
Counter Reset on Capture Event 4: 1'b0 Do Not reset Counter on Capture Event 4 (absolute time stamp); 1'b1 Reset Counter after Event 4 time-stamp has been captured (used in Difference mode operation)"]
pub type Ctrrst4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPLDEN` reader - 8:8\\]
Enable Loading of CAP1-4 registers on a Capture Event: 1'b0 Disable CAP1-4 register loads at capture Event time; 1'b1 Enable CAP1-4 register loads at capture Event time"]
pub type CapldenR = crate::BitReader;
#[doc = "Field `CAPLDEN` writer - 8:8\\]
Enable Loading of CAP1-4 registers on a Capture Event: 1'b0 Disable CAP1-4 register loads at capture Event time; 1'b1 Enable CAP1-4 register loads at capture Event time"]
pub type CapldenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVTFLTPS` reader - 13:9\\]
Event Filter prescale select: 5'b00000 divide by 1 (i.e. no prescale, by-pass the prescaler); 5'b00001 divide by 2; 5'b00010 divide by 4; 5'b00011 divide by 6; 5'b00100 divide by 8; 5'b00101 divide by 10; . . . . .; 5'b11110 divide by 60; 5'b11111 divide by 62"]
pub type EvtfltpsR = crate::FieldReader;
#[doc = "Field `EVTFLTPS` writer - 13:9\\]
Event Filter prescale select: 5'b00000 divide by 1 (i.e. no prescale, by-pass the prescaler); 5'b00001 divide by 2; 5'b00010 divide by 4; 5'b00011 divide by 6; 5'b00100 divide by 8; 5'b00101 divide by 10; . . . . .; 5'b11110 divide by 60; 5'b11111 divide by 62"]
pub type EvtfltpsW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `FREE_SOFT` reader - 15:14\\]
Emulation Control 2'b00 TSCNT Counter stops immediately on emulation suspend; 2'b01 TSCNT Counter runs until = 0; 2'b1X TSCNT Counter is unaffected by emulation suspend (Run Free)"]
pub type FreeSoftR = crate::FieldReader;
#[doc = "Field `FREE_SOFT` writer - 15:14\\]
Emulation Control 2'b00 TSCNT Counter stops immediately on emulation suspend; 2'b01 TSCNT Counter runs until = 0; 2'b1X TSCNT Counter is unaffected by emulation suspend (Run Free)"]
pub type FreeSoftW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CONT_ONESHT` reader - 16:16\\]
Continuous or Oneshot mode control: (applicable only in Capture mode) 1'b0 Operate in Continuous mode 1'b1 Operate in One-Shot mode"]
pub type ContOneshtR = crate::BitReader;
#[doc = "Field `CONT_ONESHT` writer - 16:16\\]
Continuous or Oneshot mode control: (applicable only in Capture mode) 1'b0 Operate in Continuous mode 1'b1 Operate in One-Shot mode"]
pub type ContOneshtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOPVALUE` reader - 18:17\\]
Stop value for One-Shot mode: This is the number (between 1-4) of Captures allowed to occur before the CAP(1-4) registers are frozen, i.e.Capture sequence is stopped. 2'b00 Stop after Capture Event 1; 2'b01 Stop after Capture Event 2; 2'b10 Stop after Capture Event 3; 2'b11 Stop after Capture Event 4; Notes: \\[1\\]
STOPVALUE is compared to Mod4 counter, when equal, 2 actions occur: 1- Mod4 Counter is stopped (frozen) 2- Capture Register Loads are inhibited \\[2\\]
In one shot mode, further interrupt events are blocked until we re-arm, once the number of events captured has been reached."]
pub type StopvalueR = crate::FieldReader;
#[doc = "Field `STOPVALUE` writer - 18:17\\]
Stop value for One-Shot mode: This is the number (between 1-4) of Captures allowed to occur before the CAP(1-4) registers are frozen, i.e.Capture sequence is stopped. 2'b00 Stop after Capture Event 1; 2'b01 Stop after Capture Event 2; 2'b10 Stop after Capture Event 3; 2'b11 Stop after Capture Event 4; Notes: \\[1\\]
STOPVALUE is compared to Mod4 counter, when equal, 2 actions occur: 1- Mod4 Counter is stopped (frozen) 2- Capture Register Loads are inhibited \\[2\\]
In one shot mode, further interrupt events are blocked until we re-arm, once the number of events captured has been reached."]
pub type StopvalueW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REARM_RESET` reader - 19:19\\]
One-Shot Re-arming, i.e. Wait for stop Trigger: Writing a One Arms the One-Shot sequence, i.e.: 1. Resets the Mod4 counter to zero 2. Un-freezes the Mod4 counter 3. Enables Capture Register Loads; Writing a zero has no effect. Reading always returns a 0. Note: The RE-ARM function is valid in ONESHT or CONTINOUS mode."]
pub type RearmResetR = crate::BitReader;
#[doc = "Field `REARM_RESET` writer - 19:19\\]
One-Shot Re-arming, i.e. Wait for stop Trigger: Writing a One Arms the One-Shot sequence, i.e.: 1. Resets the Mod4 counter to zero 2. Un-freezes the Mod4 counter 3. Enables Capture Register Loads; Writing a zero has no effect. Reading always returns a 0. Note: The RE-ARM function is valid in ONESHT or CONTINOUS mode."]
pub type RearmResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSCNTSTP` reader - 20:20\\]
Counter Stop (freeze) Control: 1'b0 Counter Stopped; 1'b1 Counter Free Running"]
pub type TscntstpR = crate::BitReader;
#[doc = "Field `TSCNTSTP` writer - 20:20\\]
Counter Stop (freeze) Control: 1'b0 Counter Stopped; 1'b1 Counter Free Running"]
pub type TscntstpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCI_EN` reader - 21:21\\]
Counter (TSCNT) Sync-In select mode: 1'b0 Disable Sync-In option 1'b1 Enable Counter (TSCNT) to be loaded from CNTPHS register upon either a SYNCI signal or a S/W force event."]
pub type SynciEnR = crate::BitReader;
#[doc = "Field `SYNCI_EN` writer - 21:21\\]
Counter (TSCNT) Sync-In select mode: 1'b0 Disable Sync-In option 1'b1 Enable Counter (TSCNT) to be loaded from CNTPHS register upon either a SYNCI signal or a S/W force event."]
pub type SynciEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCO_SEL` reader - 23:22\\]
Sync-Out select: 2'b00 Select Sync-In event to be the Sync-Out signal (pass through); 2'b01 Select PRD_eq event to be the Sync-Out signal; 2'b10 DISABLE Sync Out Signal; 2'b11 DISABLE Sync Out Signal; Note: Selection PRD_eq is meaningful only in APWM mode, however can still be chosen in CAP mode if user believes it to be useful."]
pub type SyncoSelR = crate::FieldReader;
#[doc = "Field `SYNCO_SEL` writer - 23:22\\]
Sync-Out select: 2'b00 Select Sync-In event to be the Sync-Out signal (pass through); 2'b01 Select PRD_eq event to be the Sync-Out signal; 2'b10 DISABLE Sync Out Signal; 2'b11 DISABLE Sync Out Signal; Note: Selection PRD_eq is meaningful only in APWM mode, however can still be chosen in CAP mode if user believes it to be useful."]
pub type SyncoSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SWSYNC` reader - 24:24\\]
Software forced Counter (TSCNT) sync'ing: 1'b0 Writing a Zero has no effect Reading will always return a zero; 1'b1 Writing a One will force a TSCNT shadow load of current ECAP module and any ECAP modules down-stream providing the SYNCO_SEL bits are 2'b00. After writing a one this bit returns to a zero. Note: This provides a convenient S/W method to synchronize some or all ECAP Timebases. In APWM mode the sync'ing can also be done via the PRD_eq event."]
pub type SwsyncR = crate::BitReader;
#[doc = "Field `SWSYNC` writer - 24:24\\]
Software forced Counter (TSCNT) sync'ing: 1'b0 Writing a Zero has no effect Reading will always return a zero; 1'b1 Writing a One will force a TSCNT shadow load of current ECAP module and any ECAP modules down-stream providing the SYNCO_SEL bits are 2'b00. After writing a one this bit returns to a zero. Note: This provides a convenient S/W method to synchronize some or all ECAP Timebases. In APWM mode the sync'ing can also be done via the PRD_eq event."]
pub type SwsyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP_APWM` reader - 25:25\\]
CAP/APWM operating mode select: 1'b0 ECAP module operates in Capture mode This mode forces the following configuration: 1- Inhibits TSCNT resets via PRD_eq event 2- Inhibits Shadow loads on CAP1 &amp; 2 registers 3- Permits User to enable CAP1-4 register load 4- CAPx/APWMx pin operates as a Capture input* 1'b1 ECAP module operates in APWM mode This mode forces the following configuration: 1- Inhibits TSCNT resets via PRD_eq event 2- Inhibits Shadow loads on CAP1 &amp; 2 registers 3- Permits User to enable CAP1-4 register load 4- CAPx/APWMx pin operates as a Capture input*; 1'b1 ECAP module operates in APWM mode This mode forces the following configuration: 1- Resets TSCNT on PRD_eq event (period boundary) 2- Permits Shadow loading on CAP1 &amp; 2 registers 3- Disables loading of Time-stamps into CAP1-4 regs 4- CAPx/APWMx pin operates as a APWM output*"]
pub type CapApwmR = crate::BitReader;
#[doc = "Field `CAP_APWM` writer - 25:25\\]
CAP/APWM operating mode select: 1'b0 ECAP module operates in Capture mode This mode forces the following configuration: 1- Inhibits TSCNT resets via PRD_eq event 2- Inhibits Shadow loads on CAP1 &amp; 2 registers 3- Permits User to enable CAP1-4 register load 4- CAPx/APWMx pin operates as a Capture input* 1'b1 ECAP module operates in APWM mode This mode forces the following configuration: 1- Inhibits TSCNT resets via PRD_eq event 2- Inhibits Shadow loads on CAP1 &amp; 2 registers 3- Permits User to enable CAP1-4 register load 4- CAPx/APWMx pin operates as a Capture input*; 1'b1 ECAP module operates in APWM mode This mode forces the following configuration: 1- Resets TSCNT on PRD_eq event (period boundary) 2- Permits Shadow loading on CAP1 &amp; 2 registers 3- Disables loading of Time-stamps into CAP1-4 regs 4- CAPx/APWMx pin operates as a APWM output*"]
pub type CapApwmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APWMPOL` reader - 26:26\\]
APWM output polarity select: 1'b0 Output is Active High (i.e. Compare value defines High time); 1'b1 Output is Active Low (i.e. Compare value defines Low time); Note: This is applicable only in APWM operating mode"]
pub type ApwmpolR = crate::BitReader;
#[doc = "Field `APWMPOL` writer - 26:26\\]
APWM output polarity select: 1'b0 Output is Active High (i.e. Compare value defines High time); 1'b1 Output is Active Low (i.e. Compare value defines Low time); Note: This is applicable only in APWM operating mode"]
pub type ApwmpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FILTER` reader - "]
pub type FilterR = crate::FieldReader;
#[doc = "Field `FILTER` writer - "]
pub type FilterW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Capture Event 1 Polarity select: 1'b0 Capture event 1 triggered on a Rising Edge (FE); 1'b1 Capture event 1 triggered on a Falling Edge (FE)"]
    #[inline(always)]
    pub fn cap1pol(&self) -> Cap1polR {
        Cap1polR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Counter Reset on Capture Event 1: 1'b0 Do Not reset Counter on Capture Event 1 (absolute time stamp); 1'b1 Reset Counter after Event 1 time-stamp has been captured (used in Difference mode operation)"]
    #[inline(always)]
    pub fn ctrrst1(&self) -> Ctrrst1R {
        Ctrrst1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Capture Event 2 Polarity select: 1'b0 Capture event 2 triggered on a Rising Edge (FE); 1'b1 Capture event 2 triggered on a Falling Edge (FE)"]
    #[inline(always)]
    pub fn cap2pol(&self) -> Cap2polR {
        Cap2polR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Counter Reset on Capture Event 2: 1'b0 Do Not reset Counter on Capture Event 2 (absolute time stamp); 1'b1 Reset Counter after Event 2 time-stamp has been captured (used in Difference mode operation)"]
    #[inline(always)]
    pub fn ctrrst2(&self) -> Ctrrst2R {
        Ctrrst2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Capture Event 3 Polarity select: 1'b0 Capture event 3 triggered on a Rising Edge (FE); 1'b1 Capture event 3 triggered on a Falling Edge (FE)"]
    #[inline(always)]
    pub fn cap3pol(&self) -> Cap3polR {
        Cap3polR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Counter Reset on Capture Event 3: 1'b0 Do Not reset Counter on Capture Event 3 (absolute time stamp); 1'b1 Reset Counter after Event 3 time-stamp has been captured (used in Difference mode operation)"]
    #[inline(always)]
    pub fn ctrrst3(&self) -> Ctrrst3R {
        Ctrrst3R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Capture Event 4 Polarity select: 1'b0 Capture event 4 triggered on a Rising Edge (FE); 1'b1 Capture event 4 triggered on a Falling Edge (FE)"]
    #[inline(always)]
    pub fn cap4pol(&self) -> Cap4polR {
        Cap4polR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Counter Reset on Capture Event 4: 1'b0 Do Not reset Counter on Capture Event 4 (absolute time stamp); 1'b1 Reset Counter after Event 4 time-stamp has been captured (used in Difference mode operation)"]
    #[inline(always)]
    pub fn ctrrst4(&self) -> Ctrrst4R {
        Ctrrst4R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable Loading of CAP1-4 registers on a Capture Event: 1'b0 Disable CAP1-4 register loads at capture Event time; 1'b1 Enable CAP1-4 register loads at capture Event time"]
    #[inline(always)]
    pub fn caplden(&self) -> CapldenR {
        CapldenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:13 - 13:9\\]
Event Filter prescale select: 5'b00000 divide by 1 (i.e. no prescale, by-pass the prescaler); 5'b00001 divide by 2; 5'b00010 divide by 4; 5'b00011 divide by 6; 5'b00100 divide by 8; 5'b00101 divide by 10; . . . . .; 5'b11110 divide by 60; 5'b11111 divide by 62"]
    #[inline(always)]
    pub fn evtfltps(&self) -> EvtfltpsR {
        EvtfltpsR::new(((self.bits >> 9) & 0x1f) as u8)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Emulation Control 2'b00 TSCNT Counter stops immediately on emulation suspend; 2'b01 TSCNT Counter runs until = 0; 2'b1X TSCNT Counter is unaffected by emulation suspend (Run Free)"]
    #[inline(always)]
    pub fn free_soft(&self) -> FreeSoftR {
        FreeSoftR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Continuous or Oneshot mode control: (applicable only in Capture mode) 1'b0 Operate in Continuous mode 1'b1 Operate in One-Shot mode"]
    #[inline(always)]
    pub fn cont_onesht(&self) -> ContOneshtR {
        ContOneshtR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - 18:17\\]
Stop value for One-Shot mode: This is the number (between 1-4) of Captures allowed to occur before the CAP(1-4) registers are frozen, i.e.Capture sequence is stopped. 2'b00 Stop after Capture Event 1; 2'b01 Stop after Capture Event 2; 2'b10 Stop after Capture Event 3; 2'b11 Stop after Capture Event 4; Notes: \\[1\\]
STOPVALUE is compared to Mod4 counter, when equal, 2 actions occur: 1- Mod4 Counter is stopped (frozen) 2- Capture Register Loads are inhibited \\[2\\]
In one shot mode, further interrupt events are blocked until we re-arm, once the number of events captured has been reached."]
    #[inline(always)]
    pub fn stopvalue(&self) -> StopvalueR {
        StopvalueR::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - 19:19\\]
One-Shot Re-arming, i.e. Wait for stop Trigger: Writing a One Arms the One-Shot sequence, i.e.: 1. Resets the Mod4 counter to zero 2. Un-freezes the Mod4 counter 3. Enables Capture Register Loads; Writing a zero has no effect. Reading always returns a 0. Note: The RE-ARM function is valid in ONESHT or CONTINOUS mode."]
    #[inline(always)]
    pub fn rearm_reset(&self) -> RearmResetR {
        RearmResetR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Counter Stop (freeze) Control: 1'b0 Counter Stopped; 1'b1 Counter Free Running"]
    #[inline(always)]
    pub fn tscntstp(&self) -> TscntstpR {
        TscntstpR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Counter (TSCNT) Sync-In select mode: 1'b0 Disable Sync-In option 1'b1 Enable Counter (TSCNT) to be loaded from CNTPHS register upon either a SYNCI signal or a S/W force event."]
    #[inline(always)]
    pub fn synci_en(&self) -> SynciEnR {
        SynciEnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Sync-Out select: 2'b00 Select Sync-In event to be the Sync-Out signal (pass through); 2'b01 Select PRD_eq event to be the Sync-Out signal; 2'b10 DISABLE Sync Out Signal; 2'b11 DISABLE Sync Out Signal; Note: Selection PRD_eq is meaningful only in APWM mode, however can still be chosen in CAP mode if user believes it to be useful."]
    #[inline(always)]
    pub fn synco_sel(&self) -> SyncoSelR {
        SyncoSelR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Software forced Counter (TSCNT) sync'ing: 1'b0 Writing a Zero has no effect Reading will always return a zero; 1'b1 Writing a One will force a TSCNT shadow load of current ECAP module and any ECAP modules down-stream providing the SYNCO_SEL bits are 2'b00. After writing a one this bit returns to a zero. Note: This provides a convenient S/W method to synchronize some or all ECAP Timebases. In APWM mode the sync'ing can also be done via the PRD_eq event."]
    #[inline(always)]
    pub fn swsync(&self) -> SwsyncR {
        SwsyncR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
CAP/APWM operating mode select: 1'b0 ECAP module operates in Capture mode This mode forces the following configuration: 1- Inhibits TSCNT resets via PRD_eq event 2- Inhibits Shadow loads on CAP1 &amp; 2 registers 3- Permits User to enable CAP1-4 register load 4- CAPx/APWMx pin operates as a Capture input* 1'b1 ECAP module operates in APWM mode This mode forces the following configuration: 1- Inhibits TSCNT resets via PRD_eq event 2- Inhibits Shadow loads on CAP1 &amp; 2 registers 3- Permits User to enable CAP1-4 register load 4- CAPx/APWMx pin operates as a Capture input*; 1'b1 ECAP module operates in APWM mode This mode forces the following configuration: 1- Resets TSCNT on PRD_eq event (period boundary) 2- Permits Shadow loading on CAP1 &amp; 2 registers 3- Disables loading of Time-stamps into CAP1-4 regs 4- CAPx/APWMx pin operates as a APWM output*"]
    #[inline(always)]
    pub fn cap_apwm(&self) -> CapApwmR {
        CapApwmR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
APWM output polarity select: 1'b0 Output is Active High (i.e. Compare value defines High time); 1'b1 Output is Active Low (i.e. Compare value defines Low time); Note: This is applicable only in APWM operating mode"]
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
Capture Event 1 Polarity select: 1'b0 Capture event 1 triggered on a Rising Edge (FE); 1'b1 Capture event 1 triggered on a Falling Edge (FE)"]
    #[inline(always)]
    #[must_use]
    pub fn cap1pol(&mut self) -> Cap1polW<CtlStsEcctlSpec> {
        Cap1polW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Counter Reset on Capture Event 1: 1'b0 Do Not reset Counter on Capture Event 1 (absolute time stamp); 1'b1 Reset Counter after Event 1 time-stamp has been captured (used in Difference mode operation)"]
    #[inline(always)]
    #[must_use]
    pub fn ctrrst1(&mut self) -> Ctrrst1W<CtlStsEcctlSpec> {
        Ctrrst1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Capture Event 2 Polarity select: 1'b0 Capture event 2 triggered on a Rising Edge (FE); 1'b1 Capture event 2 triggered on a Falling Edge (FE)"]
    #[inline(always)]
    #[must_use]
    pub fn cap2pol(&mut self) -> Cap2polW<CtlStsEcctlSpec> {
        Cap2polW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Counter Reset on Capture Event 2: 1'b0 Do Not reset Counter on Capture Event 2 (absolute time stamp); 1'b1 Reset Counter after Event 2 time-stamp has been captured (used in Difference mode operation)"]
    #[inline(always)]
    #[must_use]
    pub fn ctrrst2(&mut self) -> Ctrrst2W<CtlStsEcctlSpec> {
        Ctrrst2W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Capture Event 3 Polarity select: 1'b0 Capture event 3 triggered on a Rising Edge (FE); 1'b1 Capture event 3 triggered on a Falling Edge (FE)"]
    #[inline(always)]
    #[must_use]
    pub fn cap3pol(&mut self) -> Cap3polW<CtlStsEcctlSpec> {
        Cap3polW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Counter Reset on Capture Event 3: 1'b0 Do Not reset Counter on Capture Event 3 (absolute time stamp); 1'b1 Reset Counter after Event 3 time-stamp has been captured (used in Difference mode operation)"]
    #[inline(always)]
    #[must_use]
    pub fn ctrrst3(&mut self) -> Ctrrst3W<CtlStsEcctlSpec> {
        Ctrrst3W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Capture Event 4 Polarity select: 1'b0 Capture event 4 triggered on a Rising Edge (FE); 1'b1 Capture event 4 triggered on a Falling Edge (FE)"]
    #[inline(always)]
    #[must_use]
    pub fn cap4pol(&mut self) -> Cap4polW<CtlStsEcctlSpec> {
        Cap4polW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Counter Reset on Capture Event 4: 1'b0 Do Not reset Counter on Capture Event 4 (absolute time stamp); 1'b1 Reset Counter after Event 4 time-stamp has been captured (used in Difference mode operation)"]
    #[inline(always)]
    #[must_use]
    pub fn ctrrst4(&mut self) -> Ctrrst4W<CtlStsEcctlSpec> {
        Ctrrst4W::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable Loading of CAP1-4 registers on a Capture Event: 1'b0 Disable CAP1-4 register loads at capture Event time; 1'b1 Enable CAP1-4 register loads at capture Event time"]
    #[inline(always)]
    #[must_use]
    pub fn caplden(&mut self) -> CapldenW<CtlStsEcctlSpec> {
        CapldenW::new(self, 8)
    }
    #[doc = "Bits 9:13 - 13:9\\]
Event Filter prescale select: 5'b00000 divide by 1 (i.e. no prescale, by-pass the prescaler); 5'b00001 divide by 2; 5'b00010 divide by 4; 5'b00011 divide by 6; 5'b00100 divide by 8; 5'b00101 divide by 10; . . . . .; 5'b11110 divide by 60; 5'b11111 divide by 62"]
    #[inline(always)]
    #[must_use]
    pub fn evtfltps(&mut self) -> EvtfltpsW<CtlStsEcctlSpec> {
        EvtfltpsW::new(self, 9)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Emulation Control 2'b00 TSCNT Counter stops immediately on emulation suspend; 2'b01 TSCNT Counter runs until = 0; 2'b1X TSCNT Counter is unaffected by emulation suspend (Run Free)"]
    #[inline(always)]
    #[must_use]
    pub fn free_soft(&mut self) -> FreeSoftW<CtlStsEcctlSpec> {
        FreeSoftW::new(self, 14)
    }
    #[doc = "Bit 16 - 16:16\\]
Continuous or Oneshot mode control: (applicable only in Capture mode) 1'b0 Operate in Continuous mode 1'b1 Operate in One-Shot mode"]
    #[inline(always)]
    #[must_use]
    pub fn cont_onesht(&mut self) -> ContOneshtW<CtlStsEcctlSpec> {
        ContOneshtW::new(self, 16)
    }
    #[doc = "Bits 17:18 - 18:17\\]
Stop value for One-Shot mode: This is the number (between 1-4) of Captures allowed to occur before the CAP(1-4) registers are frozen, i.e.Capture sequence is stopped. 2'b00 Stop after Capture Event 1; 2'b01 Stop after Capture Event 2; 2'b10 Stop after Capture Event 3; 2'b11 Stop after Capture Event 4; Notes: \\[1\\]
STOPVALUE is compared to Mod4 counter, when equal, 2 actions occur: 1- Mod4 Counter is stopped (frozen) 2- Capture Register Loads are inhibited \\[2\\]
In one shot mode, further interrupt events are blocked until we re-arm, once the number of events captured has been reached."]
    #[inline(always)]
    #[must_use]
    pub fn stopvalue(&mut self) -> StopvalueW<CtlStsEcctlSpec> {
        StopvalueW::new(self, 17)
    }
    #[doc = "Bit 19 - 19:19\\]
One-Shot Re-arming, i.e. Wait for stop Trigger: Writing a One Arms the One-Shot sequence, i.e.: 1. Resets the Mod4 counter to zero 2. Un-freezes the Mod4 counter 3. Enables Capture Register Loads; Writing a zero has no effect. Reading always returns a 0. Note: The RE-ARM function is valid in ONESHT or CONTINOUS mode."]
    #[inline(always)]
    #[must_use]
    pub fn rearm_reset(&mut self) -> RearmResetW<CtlStsEcctlSpec> {
        RearmResetW::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
Counter Stop (freeze) Control: 1'b0 Counter Stopped; 1'b1 Counter Free Running"]
    #[inline(always)]
    #[must_use]
    pub fn tscntstp(&mut self) -> TscntstpW<CtlStsEcctlSpec> {
        TscntstpW::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
Counter (TSCNT) Sync-In select mode: 1'b0 Disable Sync-In option 1'b1 Enable Counter (TSCNT) to be loaded from CNTPHS register upon either a SYNCI signal or a S/W force event."]
    #[inline(always)]
    #[must_use]
    pub fn synci_en(&mut self) -> SynciEnW<CtlStsEcctlSpec> {
        SynciEnW::new(self, 21)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Sync-Out select: 2'b00 Select Sync-In event to be the Sync-Out signal (pass through); 2'b01 Select PRD_eq event to be the Sync-Out signal; 2'b10 DISABLE Sync Out Signal; 2'b11 DISABLE Sync Out Signal; Note: Selection PRD_eq is meaningful only in APWM mode, however can still be chosen in CAP mode if user believes it to be useful."]
    #[inline(always)]
    #[must_use]
    pub fn synco_sel(&mut self) -> SyncoSelW<CtlStsEcctlSpec> {
        SyncoSelW::new(self, 22)
    }
    #[doc = "Bit 24 - 24:24\\]
Software forced Counter (TSCNT) sync'ing: 1'b0 Writing a Zero has no effect Reading will always return a zero; 1'b1 Writing a One will force a TSCNT shadow load of current ECAP module and any ECAP modules down-stream providing the SYNCO_SEL bits are 2'b00. After writing a one this bit returns to a zero. Note: This provides a convenient S/W method to synchronize some or all ECAP Timebases. In APWM mode the sync'ing can also be done via the PRD_eq event."]
    #[inline(always)]
    #[must_use]
    pub fn swsync(&mut self) -> SwsyncW<CtlStsEcctlSpec> {
        SwsyncW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
CAP/APWM operating mode select: 1'b0 ECAP module operates in Capture mode This mode forces the following configuration: 1- Inhibits TSCNT resets via PRD_eq event 2- Inhibits Shadow loads on CAP1 &amp; 2 registers 3- Permits User to enable CAP1-4 register load 4- CAPx/APWMx pin operates as a Capture input* 1'b1 ECAP module operates in APWM mode This mode forces the following configuration: 1- Inhibits TSCNT resets via PRD_eq event 2- Inhibits Shadow loads on CAP1 &amp; 2 registers 3- Permits User to enable CAP1-4 register load 4- CAPx/APWMx pin operates as a Capture input*; 1'b1 ECAP module operates in APWM mode This mode forces the following configuration: 1- Resets TSCNT on PRD_eq event (period boundary) 2- Permits Shadow loading on CAP1 &amp; 2 registers 3- Disables loading of Time-stamps into CAP1-4 regs 4- CAPx/APWMx pin operates as a APWM output*"]
    #[inline(always)]
    #[must_use]
    pub fn cap_apwm(&mut self) -> CapApwmW<CtlStsEcctlSpec> {
        CapApwmW::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
APWM output polarity select: 1'b0 Output is Active High (i.e. Compare value defines High time); 1'b1 Output is Active Low (i.e. Compare value defines Low time); Note: This is applicable only in APWM operating mode"]
    #[inline(always)]
    #[must_use]
    pub fn apwmpol(&mut self) -> ApwmpolW<CtlStsEcctlSpec> {
        ApwmpolW::new(self, 26)
    }
    #[doc = "Bits 27:31"]
    #[inline(always)]
    #[must_use]
    pub fn filter(&mut self) -> FilterW<CtlStsEcctlSpec> {
        FilterW::new(self, 27)
    }
}
#[doc = "CTL_STS_ECCTL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl_sts_ecctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl_sts_ecctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlStsEcctlSpec;
impl crate::RegisterSpec for CtlStsEcctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl_sts_ecctl::R`](R) reader structure"]
impl crate::Readable for CtlStsEcctlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl_sts_ecctl::W`](W) writer structure"]
impl crate::Writable for CtlStsEcctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL_STS_ECCTL to value 0x0006_0000"]
impl crate::Resettable for CtlStsEcctlSpec {
    const RESET_VALUE: u32 = 0x0006_0000;
}
