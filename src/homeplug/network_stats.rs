use super::*;
use crate::*;

// FIXME: this should be done way more elegantly, maybe with an associated constant

pub struct NetworkStatsV1Request;
impl<'a> MessageTX<'a> for NetworkStatsV1Request {
    const MMV: MMV = MMV::HOMEPLUG_AV_1_1;
    const MMTYPE: MMType = MMType::CM_NW_STATS;
    type Response = NetworkStats<'a>;
}

pub struct NetworkStatsV2Request;
impl<'a> MessageTX<'a> for NetworkStatsV2Request {
    const MMV: MMV = MMV::HOMEPLUG_AV_2_0;
    const MMTYPE: MMType = MMType::CM_NW_STATS;
    type Response = NetworkStats<'a>;
}

#[derive(Eq, PartialEq)]
pub struct NetworkStatsEntry<'a>(pub &'a [u8], MMV);
impl core::fmt::Debug for NetworkStatsEntry<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let data = self.0;
        let sta = EtherAddr::from_slice(&data[..6]);

        let (tx, rx) = match self.1 {
            MMV::HOMEPLUG_AV_1_0 | MMV::HOMEPLUG_AV_1_1 => (data[6] as u16, data[7] as u16),
            MMV::HOMEPLUG_AV_2_0 => {
                let tx = u16::from_le_bytes([data[6], data[7]]);
                let rx = u16::from_le_bytes([data[8], data[9]]);
                (tx, rx)
            }
            _ => (0, 0),
        };

        write!(f, "STATS[sta={sta:?} tx={tx:?} rx={rx:?}]")?;
        Ok(())
    }
}

impl core::hash::Hash for NetworkStatsEntry<'_> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

#[derive(Eq, PartialEq, Hash)]
pub struct NetworkStats<'a>(pub &'a [u8]);

impl NetworkStats<'_> {
    pub fn stats(&self) -> impl ExactSizeIterator<Item = NetworkStatsEntry<'_>> + '_ {
        let count = self.payload()[0] as usize;
        let mmv = self.mmv();
        self.payload()[1..]
            .chunks_exact(10)
            .take(count)
            .map(move |d| NetworkStatsEntry(d, mmv))
    }
}

impl MessageReader for NetworkStats<'_> {
    fn bytes(&self) -> &[u8] {
        self.0
    }
}

impl<'a> From<&'a [u8]> for NetworkStats<'a> {
    fn from(data: &'a [u8]) -> Self {
        Self(data)
    }
}

impl core::fmt::Debug for NetworkStats<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        for stats in self.stats() {
            write!(f, "{stats:?}")?;
        }
        Ok(())
    }
}
