#[doc = "Register `REGS3_MAILBOX_IRQ_STATUS_CLR` reader"]
pub type R = crate::R<Regs3MailboxIrqStatusClrSpec>;
#[doc = "Register `REGS3_MAILBOX_IRQ_STATUS_CLR` writer"]
pub type W = crate::W<Regs3MailboxIrqStatusClrSpec>;
#[doc = "Field `NEWMSGSTATUSMB0` reader - 0:0\\]
1 if there are messages present in Mailbox 0 and this interrupt bit is enabled"]
pub type Newmsgstatusmb0R = crate::BitReader;
#[doc = "Field `NEWMSGSTATUSMB0` writer - 0:0\\]
1 if there are messages present in Mailbox 0 and this interrupt bit is enabled"]
pub type Newmsgstatusmb0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOTFULLSTATUSMB0` reader - 1:1\\]
1 if Mailbox 0 is not full and this interrupt bit is enabled"]
pub type Notfullstatusmb0R = crate::BitReader;
#[doc = "Field `NOTFULLSTATUSMB0` writer - 1:1\\]
1 if Mailbox 0 is not full and this interrupt bit is enabled"]
pub type Notfullstatusmb0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NEWMSGSTATUSMB1` reader - 2:2\\]
1 if there are messages present in Mailbox 1 and this interrupt bit is enabled"]
pub type Newmsgstatusmb1R = crate::BitReader;
#[doc = "Field `NEWMSGSTATUSMB1` writer - 2:2\\]
1 if there are messages present in Mailbox 1 and this interrupt bit is enabled"]
pub type Newmsgstatusmb1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOTFULLSTATUSMB1` reader - 3:3\\]
1 if Mailbox 1 is not full and this interrupt bit is enabled"]
pub type Notfullstatusmb1R = crate::BitReader;
#[doc = "Field `NOTFULLSTATUSMB1` writer - 3:3\\]
1 if Mailbox 1 is not full and this interrupt bit is enabled"]
pub type Notfullstatusmb1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NEWMSGSTATUSMB2` reader - 4:4\\]
1 if there are messages present in Mailbox 2 and this interrupt bit is enabled"]
pub type Newmsgstatusmb2R = crate::BitReader;
#[doc = "Field `NEWMSGSTATUSMB2` writer - 4:4\\]
1 if there are messages present in Mailbox 2 and this interrupt bit is enabled"]
pub type Newmsgstatusmb2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOTFULLSTATUSMB2` reader - 5:5\\]
1 if Mailbox 2 is not full and this interrupt bit is enabled"]
pub type Notfullstatusmb2R = crate::BitReader;
#[doc = "Field `NOTFULLSTATUSMB2` writer - 5:5\\]
1 if Mailbox 2 is not full and this interrupt bit is enabled"]
pub type Notfullstatusmb2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NEWMSGSTATUSMB3` reader - 6:6\\]
1 if there are messages present in Mailbox 3 and this interrupt bit is enabled"]
pub type Newmsgstatusmb3R = crate::BitReader;
#[doc = "Field `NEWMSGSTATUSMB3` writer - 6:6\\]
1 if there are messages present in Mailbox 3 and this interrupt bit is enabled"]
pub type Newmsgstatusmb3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOTFULLSTATUSMB3` reader - 7:7\\]
1 if Mailbox 3 is not full and this interrupt bit is enabled"]
pub type Notfullstatusmb3R = crate::BitReader;
#[doc = "Field `NOTFULLSTATUSMB3` writer - 7:7\\]
1 if Mailbox 3 is not full and this interrupt bit is enabled"]
pub type Notfullstatusmb3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NEWMSGSTATUSMB4` reader - 8:8\\]
1 if there are messages present in Mailbox 4 and this interrupt bit is enabled"]
pub type Newmsgstatusmb4R = crate::BitReader;
#[doc = "Field `NEWMSGSTATUSMB4` writer - 8:8\\]
1 if there are messages present in Mailbox 4 and this interrupt bit is enabled"]
pub type Newmsgstatusmb4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOTFULLSTATUSMB4` reader - 9:9\\]
1 if Mailbox 4 is not full and this interrupt bit is enabled"]
pub type Notfullstatusmb4R = crate::BitReader;
#[doc = "Field `NOTFULLSTATUSMB4` writer - 9:9\\]
1 if Mailbox 4 is not full and this interrupt bit is enabled"]
pub type Notfullstatusmb4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NEWMSGSTATUSMB5` reader - 10:10\\]
1 if there are messages present in Mailbox 5 and this interrupt bit is enabled"]
pub type Newmsgstatusmb5R = crate::BitReader;
#[doc = "Field `NEWMSGSTATUSMB5` writer - 10:10\\]
1 if there are messages present in Mailbox 5 and this interrupt bit is enabled"]
pub type Newmsgstatusmb5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOTFULLSTATUSMB5` reader - 11:11\\]
1 if Mailbox 5 is not full and this interrupt bit is enabled"]
pub type Notfullstatusmb5R = crate::BitReader;
#[doc = "Field `NOTFULLSTATUSMB5` writer - 11:11\\]
1 if Mailbox 5 is not full and this interrupt bit is enabled"]
pub type Notfullstatusmb5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NEWMSGSTATUSMB6` reader - 12:12\\]
1 if there are messages present in Mailbox 6 and this interrupt bit is enabled"]
pub type Newmsgstatusmb6R = crate::BitReader;
#[doc = "Field `NEWMSGSTATUSMB6` writer - 12:12\\]
1 if there are messages present in Mailbox 6 and this interrupt bit is enabled"]
pub type Newmsgstatusmb6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOTFULLSTATUSMB6` reader - 13:13\\]
1 if Mailbox 6 is not full and this interrupt bit is enabled"]
pub type Notfullstatusmb6R = crate::BitReader;
#[doc = "Field `NOTFULLSTATUSMB6` writer - 13:13\\]
1 if Mailbox 6 is not full and this interrupt bit is enabled"]
pub type Notfullstatusmb6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NEWMSGSTATUSMB7` reader - 14:14\\]
1 if there are messages present in Mailbox 7 and this interrupt bit is enabled"]
pub type Newmsgstatusmb7R = crate::BitReader;
#[doc = "Field `NEWMSGSTATUSMB7` writer - 14:14\\]
1 if there are messages present in Mailbox 7 and this interrupt bit is enabled"]
pub type Newmsgstatusmb7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOTFULLSTATUSMB7` reader - 15:15\\]
1 if Mailbox 7 is not full and this interrupt bit is enabled"]
pub type Notfullstatusmb7R = crate::BitReader;
#[doc = "Field `NOTFULLSTATUSMB7` writer - 15:15\\]
1 if Mailbox 7 is not full and this interrupt bit is enabled"]
pub type Notfullstatusmb7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NEWMSGSTATUSMB8` reader - 16:16\\]
1 if there are messages present in Mailbox 8 and this interrupt bit is enabled"]
pub type Newmsgstatusmb8R = crate::BitReader;
#[doc = "Field `NEWMSGSTATUSMB8` writer - 16:16\\]
1 if there are messages present in Mailbox 8 and this interrupt bit is enabled"]
pub type Newmsgstatusmb8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOTFULLSTATUSMB8` reader - 17:17\\]
1 if Mailbox 8 is not full and this interrupt bit is enabled"]
pub type Notfullstatusmb8R = crate::BitReader;
#[doc = "Field `NOTFULLSTATUSMB8` writer - 17:17\\]
1 if Mailbox 8 is not full and this interrupt bit is enabled"]
pub type Notfullstatusmb8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NEWMSGSTATUSMB9` reader - 18:18\\]
1 if there are messages present in Mailbox 9 and this interrupt bit is enabled"]
pub type Newmsgstatusmb9R = crate::BitReader;
#[doc = "Field `NEWMSGSTATUSMB9` writer - 18:18\\]
1 if there are messages present in Mailbox 9 and this interrupt bit is enabled"]
pub type Newmsgstatusmb9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOTFULLSTATUSMB9` reader - 19:19\\]
1 if Mailbox 9 is not full and this interrupt bit is enabled"]
pub type Notfullstatusmb9R = crate::BitReader;
#[doc = "Field `NOTFULLSTATUSMB9` writer - 19:19\\]
1 if Mailbox 9 is not full and this interrupt bit is enabled"]
pub type Notfullstatusmb9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NEWMSGSTATUSMB10` reader - 20:20\\]
1 if there are messages present in Mailbox 10 and this interrupt bit is enabled"]
pub type Newmsgstatusmb10R = crate::BitReader;
#[doc = "Field `NEWMSGSTATUSMB10` writer - 20:20\\]
1 if there are messages present in Mailbox 10 and this interrupt bit is enabled"]
pub type Newmsgstatusmb10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOTFULLSTATUSMB10` reader - 21:21\\]
1 if Mailbox 10 is not full and this interrupt bit is enabled"]
pub type Notfullstatusmb10R = crate::BitReader;
#[doc = "Field `NOTFULLSTATUSMB10` writer - 21:21\\]
1 if Mailbox 10 is not full and this interrupt bit is enabled"]
pub type Notfullstatusmb10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NEWMSGSTATUSMB11` reader - 22:22\\]
1 if there are messages present in Mailbox 11 and this interrupt bit is enabled"]
pub type Newmsgstatusmb11R = crate::BitReader;
#[doc = "Field `NEWMSGSTATUSMB11` writer - 22:22\\]
1 if there are messages present in Mailbox 11 and this interrupt bit is enabled"]
pub type Newmsgstatusmb11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOTFULLSTATUSMB11` reader - 23:23\\]
1 if Mailbox 11 is not full and this interrupt bit is enabled"]
pub type Notfullstatusmb11R = crate::BitReader;
#[doc = "Field `NOTFULLSTATUSMB11` writer - 23:23\\]
1 if Mailbox 11 is not full and this interrupt bit is enabled"]
pub type Notfullstatusmb11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NEWMSGSTATUSMB12` reader - 24:24\\]
1 if there are messages present in Mailbox 12 and this interrupt bit is enabled"]
pub type Newmsgstatusmb12R = crate::BitReader;
#[doc = "Field `NEWMSGSTATUSMB12` writer - 24:24\\]
1 if there are messages present in Mailbox 12 and this interrupt bit is enabled"]
pub type Newmsgstatusmb12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOTFULLSTATUSMB12` reader - 25:25\\]
1 if Mailbox 12 is not full and this interrupt bit is enabled"]
pub type Notfullstatusmb12R = crate::BitReader;
#[doc = "Field `NOTFULLSTATUSMB12` writer - 25:25\\]
1 if Mailbox 12 is not full and this interrupt bit is enabled"]
pub type Notfullstatusmb12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NEWMSGSTATUSMB13` reader - 26:26\\]
1 if there are messages present in Mailbox 13 and this interrupt bit is enabled"]
pub type Newmsgstatusmb13R = crate::BitReader;
#[doc = "Field `NEWMSGSTATUSMB13` writer - 26:26\\]
1 if there are messages present in Mailbox 13 and this interrupt bit is enabled"]
pub type Newmsgstatusmb13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOTFULLSTATUSMB13` reader - 27:27\\]
1 if Mailbox 13 is not full and this interrupt bit is enabled"]
pub type Notfullstatusmb13R = crate::BitReader;
#[doc = "Field `NOTFULLSTATUSMB13` writer - 27:27\\]
1 if Mailbox 13 is not full and this interrupt bit is enabled"]
pub type Notfullstatusmb13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NEWMSGSTATUSMB14` reader - 28:28\\]
1 if there are messages present in Mailbox 14 and this interrupt bit is enabled"]
pub type Newmsgstatusmb14R = crate::BitReader;
#[doc = "Field `NEWMSGSTATUSMB14` writer - 28:28\\]
1 if there are messages present in Mailbox 14 and this interrupt bit is enabled"]
pub type Newmsgstatusmb14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOTFULLSTATUSMB14` reader - 29:29\\]
1 if Mailbox 14 is not full and this interrupt bit is enabled"]
pub type Notfullstatusmb14R = crate::BitReader;
#[doc = "Field `NOTFULLSTATUSMB14` writer - 29:29\\]
1 if Mailbox 14 is not full and this interrupt bit is enabled"]
pub type Notfullstatusmb14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NEWMSGSTATUSMB15` reader - 30:30\\]
1 if there are messages present in Mailbox 15 and this interrupt bit is enabled"]
pub type Newmsgstatusmb15R = crate::BitReader;
#[doc = "Field `NEWMSGSTATUSMB15` writer - 30:30\\]
1 if there are messages present in Mailbox 15 and this interrupt bit is enabled"]
pub type Newmsgstatusmb15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOTFULLSTATUSMB15` reader - 31:31\\]
1 if Mailbox 15 is not full and this interrupt bit is enabled"]
pub type Notfullstatusmb15R = crate::BitReader;
#[doc = "Field `NOTFULLSTATUSMB15` writer - 31:31\\]
1 if Mailbox 15 is not full and this interrupt bit is enabled"]
pub type Notfullstatusmb15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
1 if there are messages present in Mailbox 0 and this interrupt bit is enabled"]
    #[inline(always)]
    pub fn newmsgstatusmb0(&self) -> Newmsgstatusmb0R {
        Newmsgstatusmb0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
1 if Mailbox 0 is not full and this interrupt bit is enabled"]
    #[inline(always)]
    pub fn notfullstatusmb0(&self) -> Notfullstatusmb0R {
        Notfullstatusmb0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
1 if there are messages present in Mailbox 1 and this interrupt bit is enabled"]
    #[inline(always)]
    pub fn newmsgstatusmb1(&self) -> Newmsgstatusmb1R {
        Newmsgstatusmb1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
1 if Mailbox 1 is not full and this interrupt bit is enabled"]
    #[inline(always)]
    pub fn notfullstatusmb1(&self) -> Notfullstatusmb1R {
        Notfullstatusmb1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
1 if there are messages present in Mailbox 2 and this interrupt bit is enabled"]
    #[inline(always)]
    pub fn newmsgstatusmb2(&self) -> Newmsgstatusmb2R {
        Newmsgstatusmb2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
1 if Mailbox 2 is not full and this interrupt bit is enabled"]
    #[inline(always)]
    pub fn notfullstatusmb2(&self) -> Notfullstatusmb2R {
        Notfullstatusmb2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
1 if there are messages present in Mailbox 3 and this interrupt bit is enabled"]
    #[inline(always)]
    pub fn newmsgstatusmb3(&self) -> Newmsgstatusmb3R {
        Newmsgstatusmb3R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
1 if Mailbox 3 is not full and this interrupt bit is enabled"]
    #[inline(always)]
    pub fn notfullstatusmb3(&self) -> Notfullstatusmb3R {
        Notfullstatusmb3R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
1 if there are messages present in Mailbox 4 and this interrupt bit is enabled"]
    #[inline(always)]
    pub fn newmsgstatusmb4(&self) -> Newmsgstatusmb4R {
        Newmsgstatusmb4R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
1 if Mailbox 4 is not full and this interrupt bit is enabled"]
    #[inline(always)]
    pub fn notfullstatusmb4(&self) -> Notfullstatusmb4R {
        Notfullstatusmb4R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
1 if there are messages present in Mailbox 5 and this interrupt bit is enabled"]
    #[inline(always)]
    pub fn newmsgstatusmb5(&self) -> Newmsgstatusmb5R {
        Newmsgstatusmb5R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
1 if Mailbox 5 is not full and this interrupt bit is enabled"]
    #[inline(always)]
    pub fn notfullstatusmb5(&self) -> Notfullstatusmb5R {
        Notfullstatusmb5R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
1 if there are messages present in Mailbox 6 and this interrupt bit is enabled"]
    #[inline(always)]
    pub fn newmsgstatusmb6(&self) -> Newmsgstatusmb6R {
        Newmsgstatusmb6R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
1 if Mailbox 6 is not full and this interrupt bit is enabled"]
    #[inline(always)]
    pub fn notfullstatusmb6(&self) -> Notfullstatusmb6R {
        Notfullstatusmb6R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
1 if there are messages present in Mailbox 7 and this interrupt bit is enabled"]
    #[inline(always)]
    pub fn newmsgstatusmb7(&self) -> Newmsgstatusmb7R {
        Newmsgstatusmb7R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
1 if Mailbox 7 is not full and this interrupt bit is enabled"]
    #[inline(always)]
    pub fn notfullstatusmb7(&self) -> Notfullstatusmb7R {
        Notfullstatusmb7R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
1 if there are messages present in Mailbox 8 and this interrupt bit is enabled"]
    #[inline(always)]
    pub fn newmsgstatusmb8(&self) -> Newmsgstatusmb8R {
        Newmsgstatusmb8R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
1 if Mailbox 8 is not full and this interrupt bit is enabled"]
    #[inline(always)]
    pub fn notfullstatusmb8(&self) -> Notfullstatusmb8R {
        Notfullstatusmb8R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
1 if there are messages present in Mailbox 9 and this interrupt bit is enabled"]
    #[inline(always)]
    pub fn newmsgstatusmb9(&self) -> Newmsgstatusmb9R {
        Newmsgstatusmb9R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
1 if Mailbox 9 is not full and this interrupt bit is enabled"]
    #[inline(always)]
    pub fn notfullstatusmb9(&self) -> Notfullstatusmb9R {
        Notfullstatusmb9R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
1 if there are messages present in Mailbox 10 and this interrupt bit is enabled"]
    #[inline(always)]
    pub fn newmsgstatusmb10(&self) -> Newmsgstatusmb10R {
        Newmsgstatusmb10R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
1 if Mailbox 10 is not full and this interrupt bit is enabled"]
    #[inline(always)]
    pub fn notfullstatusmb10(&self) -> Notfullstatusmb10R {
        Notfullstatusmb10R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
1 if there are messages present in Mailbox 11 and this interrupt bit is enabled"]
    #[inline(always)]
    pub fn newmsgstatusmb11(&self) -> Newmsgstatusmb11R {
        Newmsgstatusmb11R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
1 if Mailbox 11 is not full and this interrupt bit is enabled"]
    #[inline(always)]
    pub fn notfullstatusmb11(&self) -> Notfullstatusmb11R {
        Notfullstatusmb11R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
1 if there are messages present in Mailbox 12 and this interrupt bit is enabled"]
    #[inline(always)]
    pub fn newmsgstatusmb12(&self) -> Newmsgstatusmb12R {
        Newmsgstatusmb12R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
1 if Mailbox 12 is not full and this interrupt bit is enabled"]
    #[inline(always)]
    pub fn notfullstatusmb12(&self) -> Notfullstatusmb12R {
        Notfullstatusmb12R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
1 if there are messages present in Mailbox 13 and this interrupt bit is enabled"]
    #[inline(always)]
    pub fn newmsgstatusmb13(&self) -> Newmsgstatusmb13R {
        Newmsgstatusmb13R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
1 if Mailbox 13 is not full and this interrupt bit is enabled"]
    #[inline(always)]
    pub fn notfullstatusmb13(&self) -> Notfullstatusmb13R {
        Notfullstatusmb13R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
1 if there are messages present in Mailbox 14 and this interrupt bit is enabled"]
    #[inline(always)]
    pub fn newmsgstatusmb14(&self) -> Newmsgstatusmb14R {
        Newmsgstatusmb14R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
1 if Mailbox 14 is not full and this interrupt bit is enabled"]
    #[inline(always)]
    pub fn notfullstatusmb14(&self) -> Notfullstatusmb14R {
        Notfullstatusmb14R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
1 if there are messages present in Mailbox 15 and this interrupt bit is enabled"]
    #[inline(always)]
    pub fn newmsgstatusmb15(&self) -> Newmsgstatusmb15R {
        Newmsgstatusmb15R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
1 if Mailbox 15 is not full and this interrupt bit is enabled"]
    #[inline(always)]
    pub fn notfullstatusmb15(&self) -> Notfullstatusmb15R {
        Notfullstatusmb15R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
1 if there are messages present in Mailbox 0 and this interrupt bit is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn newmsgstatusmb0(&mut self) -> Newmsgstatusmb0W<Regs3MailboxIrqStatusClrSpec> {
        Newmsgstatusmb0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
1 if Mailbox 0 is not full and this interrupt bit is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn notfullstatusmb0(&mut self) -> Notfullstatusmb0W<Regs3MailboxIrqStatusClrSpec> {
        Notfullstatusmb0W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
1 if there are messages present in Mailbox 1 and this interrupt bit is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn newmsgstatusmb1(&mut self) -> Newmsgstatusmb1W<Regs3MailboxIrqStatusClrSpec> {
        Newmsgstatusmb1W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
1 if Mailbox 1 is not full and this interrupt bit is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn notfullstatusmb1(&mut self) -> Notfullstatusmb1W<Regs3MailboxIrqStatusClrSpec> {
        Notfullstatusmb1W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
1 if there are messages present in Mailbox 2 and this interrupt bit is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn newmsgstatusmb2(&mut self) -> Newmsgstatusmb2W<Regs3MailboxIrqStatusClrSpec> {
        Newmsgstatusmb2W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
1 if Mailbox 2 is not full and this interrupt bit is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn notfullstatusmb2(&mut self) -> Notfullstatusmb2W<Regs3MailboxIrqStatusClrSpec> {
        Notfullstatusmb2W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
1 if there are messages present in Mailbox 3 and this interrupt bit is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn newmsgstatusmb3(&mut self) -> Newmsgstatusmb3W<Regs3MailboxIrqStatusClrSpec> {
        Newmsgstatusmb3W::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
1 if Mailbox 3 is not full and this interrupt bit is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn notfullstatusmb3(&mut self) -> Notfullstatusmb3W<Regs3MailboxIrqStatusClrSpec> {
        Notfullstatusmb3W::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
1 if there are messages present in Mailbox 4 and this interrupt bit is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn newmsgstatusmb4(&mut self) -> Newmsgstatusmb4W<Regs3MailboxIrqStatusClrSpec> {
        Newmsgstatusmb4W::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
1 if Mailbox 4 is not full and this interrupt bit is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn notfullstatusmb4(&mut self) -> Notfullstatusmb4W<Regs3MailboxIrqStatusClrSpec> {
        Notfullstatusmb4W::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
1 if there are messages present in Mailbox 5 and this interrupt bit is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn newmsgstatusmb5(&mut self) -> Newmsgstatusmb5W<Regs3MailboxIrqStatusClrSpec> {
        Newmsgstatusmb5W::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
1 if Mailbox 5 is not full and this interrupt bit is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn notfullstatusmb5(&mut self) -> Notfullstatusmb5W<Regs3MailboxIrqStatusClrSpec> {
        Notfullstatusmb5W::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
1 if there are messages present in Mailbox 6 and this interrupt bit is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn newmsgstatusmb6(&mut self) -> Newmsgstatusmb6W<Regs3MailboxIrqStatusClrSpec> {
        Newmsgstatusmb6W::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
1 if Mailbox 6 is not full and this interrupt bit is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn notfullstatusmb6(&mut self) -> Notfullstatusmb6W<Regs3MailboxIrqStatusClrSpec> {
        Notfullstatusmb6W::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
1 if there are messages present in Mailbox 7 and this interrupt bit is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn newmsgstatusmb7(&mut self) -> Newmsgstatusmb7W<Regs3MailboxIrqStatusClrSpec> {
        Newmsgstatusmb7W::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
1 if Mailbox 7 is not full and this interrupt bit is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn notfullstatusmb7(&mut self) -> Notfullstatusmb7W<Regs3MailboxIrqStatusClrSpec> {
        Notfullstatusmb7W::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
1 if there are messages present in Mailbox 8 and this interrupt bit is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn newmsgstatusmb8(&mut self) -> Newmsgstatusmb8W<Regs3MailboxIrqStatusClrSpec> {
        Newmsgstatusmb8W::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
1 if Mailbox 8 is not full and this interrupt bit is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn notfullstatusmb8(&mut self) -> Notfullstatusmb8W<Regs3MailboxIrqStatusClrSpec> {
        Notfullstatusmb8W::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
1 if there are messages present in Mailbox 9 and this interrupt bit is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn newmsgstatusmb9(&mut self) -> Newmsgstatusmb9W<Regs3MailboxIrqStatusClrSpec> {
        Newmsgstatusmb9W::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
1 if Mailbox 9 is not full and this interrupt bit is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn notfullstatusmb9(&mut self) -> Notfullstatusmb9W<Regs3MailboxIrqStatusClrSpec> {
        Notfullstatusmb9W::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
1 if there are messages present in Mailbox 10 and this interrupt bit is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn newmsgstatusmb10(&mut self) -> Newmsgstatusmb10W<Regs3MailboxIrqStatusClrSpec> {
        Newmsgstatusmb10W::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
1 if Mailbox 10 is not full and this interrupt bit is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn notfullstatusmb10(&mut self) -> Notfullstatusmb10W<Regs3MailboxIrqStatusClrSpec> {
        Notfullstatusmb10W::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
1 if there are messages present in Mailbox 11 and this interrupt bit is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn newmsgstatusmb11(&mut self) -> Newmsgstatusmb11W<Regs3MailboxIrqStatusClrSpec> {
        Newmsgstatusmb11W::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
1 if Mailbox 11 is not full and this interrupt bit is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn notfullstatusmb11(&mut self) -> Notfullstatusmb11W<Regs3MailboxIrqStatusClrSpec> {
        Notfullstatusmb11W::new(self, 23)
    }
    #[doc = "Bit 24 - 24:24\\]
1 if there are messages present in Mailbox 12 and this interrupt bit is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn newmsgstatusmb12(&mut self) -> Newmsgstatusmb12W<Regs3MailboxIrqStatusClrSpec> {
        Newmsgstatusmb12W::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
1 if Mailbox 12 is not full and this interrupt bit is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn notfullstatusmb12(&mut self) -> Notfullstatusmb12W<Regs3MailboxIrqStatusClrSpec> {
        Notfullstatusmb12W::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
1 if there are messages present in Mailbox 13 and this interrupt bit is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn newmsgstatusmb13(&mut self) -> Newmsgstatusmb13W<Regs3MailboxIrqStatusClrSpec> {
        Newmsgstatusmb13W::new(self, 26)
    }
    #[doc = "Bit 27 - 27:27\\]
1 if Mailbox 13 is not full and this interrupt bit is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn notfullstatusmb13(&mut self) -> Notfullstatusmb13W<Regs3MailboxIrqStatusClrSpec> {
        Notfullstatusmb13W::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
1 if there are messages present in Mailbox 14 and this interrupt bit is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn newmsgstatusmb14(&mut self) -> Newmsgstatusmb14W<Regs3MailboxIrqStatusClrSpec> {
        Newmsgstatusmb14W::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
1 if Mailbox 14 is not full and this interrupt bit is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn notfullstatusmb14(&mut self) -> Notfullstatusmb14W<Regs3MailboxIrqStatusClrSpec> {
        Notfullstatusmb14W::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
1 if there are messages present in Mailbox 15 and this interrupt bit is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn newmsgstatusmb15(&mut self) -> Newmsgstatusmb15W<Regs3MailboxIrqStatusClrSpec> {
        Newmsgstatusmb15W::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
1 if Mailbox 15 is not full and this interrupt bit is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn notfullstatusmb15(&mut self) -> Notfullstatusmb15W<Regs3MailboxIrqStatusClrSpec> {
        Notfullstatusmb15W::new(self, 31)
    }
}
#[doc = "The interrupt status register has the status for each event that may be responsible for the generation of an interrupt to the corresponding user combined with the corresponding MASK information. Software may also write 1 to a given bit to clear this bit. However, if the hardware still has pending, enabled events, the interrupt will fire again in two cycles. Write 0 has no effect.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs3_mailbox_irq_status_clr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs3_mailbox_irq_status_clr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regs3MailboxIrqStatusClrSpec;
impl crate::RegisterSpec for Regs3MailboxIrqStatusClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs3_mailbox_irq_status_clr::R`](R) reader structure"]
impl crate::Readable for Regs3MailboxIrqStatusClrSpec {}
#[doc = "`write(|w| ..)` method takes [`regs3_mailbox_irq_status_clr::W`](W) writer structure"]
impl crate::Writable for Regs3MailboxIrqStatusClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS3_MAILBOX_IRQ_STATUS_CLR to value 0"]
impl crate::Resettable for Regs3MailboxIrqStatusClrSpec {
    const RESET_VALUE: u32 = 0;
}
