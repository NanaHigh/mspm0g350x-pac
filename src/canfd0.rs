#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x6800],
    pwren: Pwren,
    rstctl: Rstctl,
    _reserved2: [u8; 0x0c],
    stat: Stat,
    _reserved3: [u8; 0x07e8],
    mcan_crel: McanCrel,
    mcan_endn: McanEndn,
    _reserved5: [u8; 0x04],
    mcan_dbtp: McanDbtp,
    mcan_test: McanTest,
    mcan_rwd: McanRwd,
    mcan_cccr: McanCccr,
    mcan_nbtp: McanNbtp,
    mcan_tscc: McanTscc,
    mcan_tscv: McanTscv,
    mcan_tocc: McanTocc,
    mcan_tocv: McanTocv,
    _reserved14: [u8; 0x10],
    mcan_ecr: McanEcr,
    mcan_psr: McanPsr,
    mcan_tdcr: McanTdcr,
    _reserved17: [u8; 0x04],
    mcan_ir: McanIr,
    mcan_ie: McanIe,
    mcan_ils: McanIls,
    mcan_ile: McanIle,
    _reserved21: [u8; 0x20],
    mcan_gfc: McanGfc,
    mcan_sidfc: McanSidfc,
    mcan_xidfc: McanXidfc,
    _reserved24: [u8; 0x04],
    mcan_xidam: McanXidam,
    mcan_hpms: McanHpms,
    mcan_ndat1: McanNdat1,
    mcan_ndat2: McanNdat2,
    mcan_rxf0c: McanRxf0c,
    mcan_rxf0s: McanRxf0s,
    mcan_rxf0a: McanRxf0a,
    mcan_rxbc: McanRxbc,
    mcan_rxf1c: McanRxf1c,
    mcan_rxf1s: McanRxf1s,
    mcan_rxf1a: McanRxf1a,
    mcan_rxesc: McanRxesc,
    mcan_txbc: McanTxbc,
    mcan_txfqs: McanTxfqs,
    mcan_txesc: McanTxesc,
    mcan_txbrp: McanTxbrp,
    mcan_txbar: McanTxbar,
    mcan_txbcr: McanTxbcr,
    mcan_txbto: McanTxbto,
    mcan_txbcf: McanTxbcf,
    mcan_txbtie: McanTxbtie,
    mcan_txbcie: McanTxbcie,
    _reserved46: [u8; 0x08],
    mcan_txefc: McanTxefc,
    mcan_txefs: McanTxefs,
    mcan_txefa: McanTxefa,
    _reserved49: [u8; 0x0104],
    mcanss_pid: McanssPid,
    mcanss_ctrl: McanssCtrl,
    mcanss_stat: McanssStat,
    mcanss_ics: McanssIcs,
    mcanss_irs: McanssIrs,
    mcanss_iecs: McanssIecs,
    mcanss_ie: McanssIe,
    mcanss_ies: McanssIes,
    mcanss_eoi: McanssEoi,
    mcanss_ext_ts_prescaler: McanssExtTsPrescaler,
    mcanss_ext_ts_unserviced_intr_cntr: McanssExtTsUnservicedIntrCntr,
    _reserved60: [u8; 0x01d4],
    mcanerr_rev: McanerrRev,
    _reserved61: [u8; 0x04],
    mcanerr_vector: McanerrVector,
    mcanerr_stat: McanerrStat,
    mcanerr_wrap_rev: McanerrWrapRev,
    mcanerr_ctrl: McanerrCtrl,
    mcanerr_err_ctrl1: McanerrErrCtrl1,
    mcanerr_err_ctrl2: McanerrErrCtrl2,
    mcanerr_err_stat1: McanerrErrStat1,
    mcanerr_err_stat2: McanerrErrStat2,
    mcanerr_err_stat3: McanerrErrStat3,
    _reserved70: [u8; 0x10],
    mcanerr_sec_eoi: McanerrSecEoi,
    mcanerr_sec_status: McanerrSecStatus,
    _reserved72: [u8; 0x3c],
    mcanerr_sec_enable_set: McanerrSecEnableSet,
    _reserved73: [u8; 0x3c],
    mcanerr_sec_enable_clr: McanerrSecEnableClr,
    _reserved74: [u8; 0x78],
    mcanerr_ded_eoi: McanerrDedEoi,
    mcanerr_ded_status: McanerrDedStatus,
    _reserved76: [u8; 0x3c],
    mcanerr_ded_enable_set: McanerrDedEnableSet,
    _reserved77: [u8; 0x3c],
    mcanerr_ded_enable_clr: McanerrDedEnableClr,
    _reserved78: [u8; 0x3c],
    mcanerr_aggr_enable_set: McanerrAggrEnableSet,
    mcanerr_aggr_enable_clr: McanerrAggrEnableClr,
    mcanerr_aggr_status_set: McanerrAggrStatusSet,
    mcanerr_aggr_status_clr: McanerrAggrStatusClr,
    _reserved82: [u8; 0x0210],
    iidx: Iidx,
    _reserved83: [u8; 0x04],
    imask: Imask,
    _reserved84: [u8; 0x04],
    ris: Ris,
    _reserved85: [u8; 0x04],
    mis: Mis,
    _reserved86: [u8; 0x04],
    iset: Iset,
    _reserved87: [u8; 0x04],
    iclr: Iclr,
    _reserved88: [u8; 0x94],
    evt_mode: EvtMode,
    _reserved89: [u8; 0x18],
    desc: Desc,
    mcanss_clken: McanssClken,
    mcanss_clkdiv: McanssClkdiv,
    mcanss_clkctl: McanssClkctl,
    mcanss_clksts: McanssClksts,
}
impl RegisterBlock {
    #[doc = "0x6800 - Power enable"]
    #[inline(always)]
    pub const fn pwren(&self) -> &Pwren {
        &self.pwren
    }
    #[doc = "0x6804 - Reset Control"]
    #[inline(always)]
    pub const fn rstctl(&self) -> &Rstctl {
        &self.rstctl
    }
    #[doc = "0x6814 - Status Register"]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
    #[doc = "0x7000 - MCAN Core Release Register"]
    #[inline(always)]
    pub const fn mcan_crel(&self) -> &McanCrel {
        &self.mcan_crel
    }
    #[doc = "0x7004 - MCAN Endian Register"]
    #[inline(always)]
    pub const fn mcan_endn(&self) -> &McanEndn {
        &self.mcan_endn
    }
    #[doc = "0x700c - MCAN Data Bit Timing and Prescaler Register"]
    #[inline(always)]
    pub const fn mcan_dbtp(&self) -> &McanDbtp {
        &self.mcan_dbtp
    }
    #[doc = "0x7010 - MCAN Test Register"]
    #[inline(always)]
    pub const fn mcan_test(&self) -> &McanTest {
        &self.mcan_test
    }
    #[doc = "0x7014 - MCAN RAM Watchdog"]
    #[inline(always)]
    pub const fn mcan_rwd(&self) -> &McanRwd {
        &self.mcan_rwd
    }
    #[doc = "0x7018 - MCAN CC Control Register"]
    #[inline(always)]
    pub const fn mcan_cccr(&self) -> &McanCccr {
        &self.mcan_cccr
    }
    #[doc = "0x701c - MCAN Nominal Bit Timing and Prescaler Register"]
    #[inline(always)]
    pub const fn mcan_nbtp(&self) -> &McanNbtp {
        &self.mcan_nbtp
    }
    #[doc = "0x7020 - MCAN Timestamp Counter Configuration"]
    #[inline(always)]
    pub const fn mcan_tscc(&self) -> &McanTscc {
        &self.mcan_tscc
    }
    #[doc = "0x7024 - MCAN Timestamp Counter Value"]
    #[inline(always)]
    pub const fn mcan_tscv(&self) -> &McanTscv {
        &self.mcan_tscv
    }
    #[doc = "0x7028 - MCAN Timeout Counter Configuration"]
    #[inline(always)]
    pub const fn mcan_tocc(&self) -> &McanTocc {
        &self.mcan_tocc
    }
    #[doc = "0x702c - MCAN Timeout Counter Value"]
    #[inline(always)]
    pub const fn mcan_tocv(&self) -> &McanTocv {
        &self.mcan_tocv
    }
    #[doc = "0x7040 - MCAN Error Counter Register"]
    #[inline(always)]
    pub const fn mcan_ecr(&self) -> &McanEcr {
        &self.mcan_ecr
    }
    #[doc = "0x7044 - MCAN Protocol Status Register"]
    #[inline(always)]
    pub const fn mcan_psr(&self) -> &McanPsr {
        &self.mcan_psr
    }
    #[doc = "0x7048 - MCAN Transmitter Delay Compensation Register"]
    #[inline(always)]
    pub const fn mcan_tdcr(&self) -> &McanTdcr {
        &self.mcan_tdcr
    }
    #[doc = "0x7050 - MCAN Interrupt Register"]
    #[inline(always)]
    pub const fn mcan_ir(&self) -> &McanIr {
        &self.mcan_ir
    }
    #[doc = "0x7054 - MCAN Interrupt Enable"]
    #[inline(always)]
    pub const fn mcan_ie(&self) -> &McanIe {
        &self.mcan_ie
    }
    #[doc = "0x7058 - MCAN Interrupt Line Select"]
    #[inline(always)]
    pub const fn mcan_ils(&self) -> &McanIls {
        &self.mcan_ils
    }
    #[doc = "0x705c - MCAN Interrupt Line Enable"]
    #[inline(always)]
    pub const fn mcan_ile(&self) -> &McanIle {
        &self.mcan_ile
    }
    #[doc = "0x7080 - MCAN Global Filter Configuration"]
    #[inline(always)]
    pub const fn mcan_gfc(&self) -> &McanGfc {
        &self.mcan_gfc
    }
    #[doc = "0x7084 - MCAN Standard ID Filter Configuration"]
    #[inline(always)]
    pub const fn mcan_sidfc(&self) -> &McanSidfc {
        &self.mcan_sidfc
    }
    #[doc = "0x7088 - MCAN Extended ID Filter Configuration"]
    #[inline(always)]
    pub const fn mcan_xidfc(&self) -> &McanXidfc {
        &self.mcan_xidfc
    }
    #[doc = "0x7090 - MCAN Extended ID and Mask"]
    #[inline(always)]
    pub const fn mcan_xidam(&self) -> &McanXidam {
        &self.mcan_xidam
    }
    #[doc = "0x7094 - MCAN High Priority Message Status"]
    #[inline(always)]
    pub const fn mcan_hpms(&self) -> &McanHpms {
        &self.mcan_hpms
    }
    #[doc = "0x7098 - MCAN New Data 1"]
    #[inline(always)]
    pub const fn mcan_ndat1(&self) -> &McanNdat1 {
        &self.mcan_ndat1
    }
    #[doc = "0x709c - MCAN New Data 2"]
    #[inline(always)]
    pub const fn mcan_ndat2(&self) -> &McanNdat2 {
        &self.mcan_ndat2
    }
    #[doc = "0x70a0 - MCAN Rx FIFO 0 Configuration"]
    #[inline(always)]
    pub const fn mcan_rxf0c(&self) -> &McanRxf0c {
        &self.mcan_rxf0c
    }
    #[doc = "0x70a4 - MCAN Rx FIFO 0 Status"]
    #[inline(always)]
    pub const fn mcan_rxf0s(&self) -> &McanRxf0s {
        &self.mcan_rxf0s
    }
    #[doc = "0x70a8 - MCAN Rx FIFO 0 Acknowledge"]
    #[inline(always)]
    pub const fn mcan_rxf0a(&self) -> &McanRxf0a {
        &self.mcan_rxf0a
    }
    #[doc = "0x70ac - MCAN Rx Buffer Configuration"]
    #[inline(always)]
    pub const fn mcan_rxbc(&self) -> &McanRxbc {
        &self.mcan_rxbc
    }
    #[doc = "0x70b0 - MCAN Rx FIFO 1 Configuration"]
    #[inline(always)]
    pub const fn mcan_rxf1c(&self) -> &McanRxf1c {
        &self.mcan_rxf1c
    }
    #[doc = "0x70b4 - MCAN Rx FIFO 1 Status"]
    #[inline(always)]
    pub const fn mcan_rxf1s(&self) -> &McanRxf1s {
        &self.mcan_rxf1s
    }
    #[doc = "0x70b8 - MCAN Rx FIFO 1 Acknowledge"]
    #[inline(always)]
    pub const fn mcan_rxf1a(&self) -> &McanRxf1a {
        &self.mcan_rxf1a
    }
    #[doc = "0x70bc - MCAN Rx Buffer / FIFO Element Size Configuration"]
    #[inline(always)]
    pub const fn mcan_rxesc(&self) -> &McanRxesc {
        &self.mcan_rxesc
    }
    #[doc = "0x70c0 - MCAN Tx Buffer Configuration"]
    #[inline(always)]
    pub const fn mcan_txbc(&self) -> &McanTxbc {
        &self.mcan_txbc
    }
    #[doc = "0x70c4 - MCAN Tx FIFO / Queue Status"]
    #[inline(always)]
    pub const fn mcan_txfqs(&self) -> &McanTxfqs {
        &self.mcan_txfqs
    }
    #[doc = "0x70c8 - MCAN Tx Buffer Element Size Configuration"]
    #[inline(always)]
    pub const fn mcan_txesc(&self) -> &McanTxesc {
        &self.mcan_txesc
    }
    #[doc = "0x70cc - MCAN Tx Buffer Request Pending"]
    #[inline(always)]
    pub const fn mcan_txbrp(&self) -> &McanTxbrp {
        &self.mcan_txbrp
    }
    #[doc = "0x70d0 - MCAN Tx Buffer Add Request"]
    #[inline(always)]
    pub const fn mcan_txbar(&self) -> &McanTxbar {
        &self.mcan_txbar
    }
    #[doc = "0x70d4 - MCAN Tx Buffer Cancellation Request"]
    #[inline(always)]
    pub const fn mcan_txbcr(&self) -> &McanTxbcr {
        &self.mcan_txbcr
    }
    #[doc = "0x70d8 - MCAN Tx Buffer Transmission Occurred"]
    #[inline(always)]
    pub const fn mcan_txbto(&self) -> &McanTxbto {
        &self.mcan_txbto
    }
    #[doc = "0x70dc - MCAN Tx Buffer Cancellation Finished"]
    #[inline(always)]
    pub const fn mcan_txbcf(&self) -> &McanTxbcf {
        &self.mcan_txbcf
    }
    #[doc = "0x70e0 - MCAN Tx Buffer Transmission Interrupt Enable"]
    #[inline(always)]
    pub const fn mcan_txbtie(&self) -> &McanTxbtie {
        &self.mcan_txbtie
    }
    #[doc = "0x70e4 - MCAN Tx Buffer Cancellation Finished Interrupt Enable"]
    #[inline(always)]
    pub const fn mcan_txbcie(&self) -> &McanTxbcie {
        &self.mcan_txbcie
    }
    #[doc = "0x70f0 - MCAN Tx Event FIFO Configuration"]
    #[inline(always)]
    pub const fn mcan_txefc(&self) -> &McanTxefc {
        &self.mcan_txefc
    }
    #[doc = "0x70f4 - MCAN Tx Event FIFO Status"]
    #[inline(always)]
    pub const fn mcan_txefs(&self) -> &McanTxefs {
        &self.mcan_txefs
    }
    #[doc = "0x70f8 - MCAN Tx Event FIFO Acknowledge"]
    #[inline(always)]
    pub const fn mcan_txefa(&self) -> &McanTxefa {
        &self.mcan_txefa
    }
    #[doc = "0x7200 - MCAN Subsystem Revision Register"]
    #[inline(always)]
    pub const fn mcanss_pid(&self) -> &McanssPid {
        &self.mcanss_pid
    }
    #[doc = "0x7204 - MCAN Subsystem Control Register"]
    #[inline(always)]
    pub const fn mcanss_ctrl(&self) -> &McanssCtrl {
        &self.mcanss_ctrl
    }
    #[doc = "0x7208 - MCAN Subsystem Status Register"]
    #[inline(always)]
    pub const fn mcanss_stat(&self) -> &McanssStat {
        &self.mcanss_stat
    }
    #[doc = "0x720c - MCAN Subsystem Interrupt Clear Shadow Register"]
    #[inline(always)]
    pub const fn mcanss_ics(&self) -> &McanssIcs {
        &self.mcanss_ics
    }
    #[doc = "0x7210 - MCAN Subsystem Interrupt Raw Satus Register"]
    #[inline(always)]
    pub const fn mcanss_irs(&self) -> &McanssIrs {
        &self.mcanss_irs
    }
    #[doc = "0x7214 - MCAN Subsystem Interrupt Enable Clear Shadow Register"]
    #[inline(always)]
    pub const fn mcanss_iecs(&self) -> &McanssIecs {
        &self.mcanss_iecs
    }
    #[doc = "0x7218 - MCAN Subsystem Interrupt Enable Register"]
    #[inline(always)]
    pub const fn mcanss_ie(&self) -> &McanssIe {
        &self.mcanss_ie
    }
    #[doc = "0x721c - MCAN Subsystem Interrupt Enable Status"]
    #[inline(always)]
    pub const fn mcanss_ies(&self) -> &McanssIes {
        &self.mcanss_ies
    }
    #[doc = "0x7220 - MCAN Subsystem End of Interrupt"]
    #[inline(always)]
    pub const fn mcanss_eoi(&self) -> &McanssEoi {
        &self.mcanss_eoi
    }
    #[doc = "0x7224 - MCAN Subsystem External Timestamp Prescaler 0"]
    #[inline(always)]
    pub const fn mcanss_ext_ts_prescaler(&self) -> &McanssExtTsPrescaler {
        &self.mcanss_ext_ts_prescaler
    }
    #[doc = "0x7228 - MCAN Subsystem External Timestamp Unserviced Interrupts Counter"]
    #[inline(always)]
    pub const fn mcanss_ext_ts_unserviced_intr_cntr(&self) -> &McanssExtTsUnservicedIntrCntr {
        &self.mcanss_ext_ts_unserviced_intr_cntr
    }
    #[doc = "0x7400 - MCAN Error Aggregator Revision Register"]
    #[inline(always)]
    pub const fn mcanerr_rev(&self) -> &McanerrRev {
        &self.mcanerr_rev
    }
    #[doc = "0x7408 - MCAN ECC Vector Register"]
    #[inline(always)]
    pub const fn mcanerr_vector(&self) -> &McanerrVector {
        &self.mcanerr_vector
    }
    #[doc = "0x740c - MCAN Error Misc Status"]
    #[inline(always)]
    pub const fn mcanerr_stat(&self) -> &McanerrStat {
        &self.mcanerr_stat
    }
    #[doc = "0x7410 - MCAN ECC Wrapper Revision Register"]
    #[inline(always)]
    pub const fn mcanerr_wrap_rev(&self) -> &McanerrWrapRev {
        &self.mcanerr_wrap_rev
    }
    #[doc = "0x7414 - MCAN ECC Control"]
    #[inline(always)]
    pub const fn mcanerr_ctrl(&self) -> &McanerrCtrl {
        &self.mcanerr_ctrl
    }
    #[doc = "0x7418 - MCAN ECC Error Control 1 Register"]
    #[inline(always)]
    pub const fn mcanerr_err_ctrl1(&self) -> &McanerrErrCtrl1 {
        &self.mcanerr_err_ctrl1
    }
    #[doc = "0x741c - MCAN ECC Error Control 2 Register"]
    #[inline(always)]
    pub const fn mcanerr_err_ctrl2(&self) -> &McanerrErrCtrl2 {
        &self.mcanerr_err_ctrl2
    }
    #[doc = "0x7420 - MCAN ECC Error Status 1 Register"]
    #[inline(always)]
    pub const fn mcanerr_err_stat1(&self) -> &McanerrErrStat1 {
        &self.mcanerr_err_stat1
    }
    #[doc = "0x7424 - MCAN ECC Error Status 2 Register"]
    #[inline(always)]
    pub const fn mcanerr_err_stat2(&self) -> &McanerrErrStat2 {
        &self.mcanerr_err_stat2
    }
    #[doc = "0x7428 - MCAN ECC Error Status 3 Register"]
    #[inline(always)]
    pub const fn mcanerr_err_stat3(&self) -> &McanerrErrStat3 {
        &self.mcanerr_err_stat3
    }
    #[doc = "0x743c - MCAN Single Error Corrected End of Interrupt Register"]
    #[inline(always)]
    pub const fn mcanerr_sec_eoi(&self) -> &McanerrSecEoi {
        &self.mcanerr_sec_eoi
    }
    #[doc = "0x7440 - MCAN Single Error Corrected Interrupt Status Register"]
    #[inline(always)]
    pub const fn mcanerr_sec_status(&self) -> &McanerrSecStatus {
        &self.mcanerr_sec_status
    }
    #[doc = "0x7480 - MCAN Single Error Corrected Interrupt Enable Set Register"]
    #[inline(always)]
    pub const fn mcanerr_sec_enable_set(&self) -> &McanerrSecEnableSet {
        &self.mcanerr_sec_enable_set
    }
    #[doc = "0x74c0 - MCAN Single Error Corrected Interrupt Enable Clear Register"]
    #[inline(always)]
    pub const fn mcanerr_sec_enable_clr(&self) -> &McanerrSecEnableClr {
        &self.mcanerr_sec_enable_clr
    }
    #[doc = "0x753c - MCAN Double Error Detected End of Interrupt Register"]
    #[inline(always)]
    pub const fn mcanerr_ded_eoi(&self) -> &McanerrDedEoi {
        &self.mcanerr_ded_eoi
    }
    #[doc = "0x7540 - MCAN Double Error Detected Interrupt Status Register"]
    #[inline(always)]
    pub const fn mcanerr_ded_status(&self) -> &McanerrDedStatus {
        &self.mcanerr_ded_status
    }
    #[doc = "0x7580 - MCAN Double Error Detected Interrupt Enable Set Register"]
    #[inline(always)]
    pub const fn mcanerr_ded_enable_set(&self) -> &McanerrDedEnableSet {
        &self.mcanerr_ded_enable_set
    }
    #[doc = "0x75c0 - MCAN Double Error Detected Interrupt Enable Clear Register"]
    #[inline(always)]
    pub const fn mcanerr_ded_enable_clr(&self) -> &McanerrDedEnableClr {
        &self.mcanerr_ded_enable_clr
    }
    #[doc = "0x7600 - MCAN Error Aggregator Enable Set Register"]
    #[inline(always)]
    pub const fn mcanerr_aggr_enable_set(&self) -> &McanerrAggrEnableSet {
        &self.mcanerr_aggr_enable_set
    }
    #[doc = "0x7604 - MCAN Error Aggregator Enable Clear Register"]
    #[inline(always)]
    pub const fn mcanerr_aggr_enable_clr(&self) -> &McanerrAggrEnableClr {
        &self.mcanerr_aggr_enable_clr
    }
    #[doc = "0x7608 - MCAN Error Aggregator Status Set Register"]
    #[inline(always)]
    pub const fn mcanerr_aggr_status_set(&self) -> &McanerrAggrStatusSet {
        &self.mcanerr_aggr_status_set
    }
    #[doc = "0x760c - MCAN Error Aggregator Status Clear Register"]
    #[inline(always)]
    pub const fn mcanerr_aggr_status_clr(&self) -> &McanerrAggrStatusClr {
        &self.mcanerr_aggr_status_clr
    }
    #[doc = "0x7820 - Interrupt Index Register"]
    #[inline(always)]
    pub const fn iidx(&self) -> &Iidx {
        &self.iidx
    }
    #[doc = "0x7828 - Interrupt mask"]
    #[inline(always)]
    pub const fn imask(&self) -> &Imask {
        &self.imask
    }
    #[doc = "0x7830 - Raw interrupt status"]
    #[inline(always)]
    pub const fn ris(&self) -> &Ris {
        &self.ris
    }
    #[doc = "0x7838 - Masked interrupt status"]
    #[inline(always)]
    pub const fn mis(&self) -> &Mis {
        &self.mis
    }
    #[doc = "0x7840 - Interrupt set"]
    #[inline(always)]
    pub const fn iset(&self) -> &Iset {
        &self.iset
    }
    #[doc = "0x7848 - Interrupt clear"]
    #[inline(always)]
    pub const fn iclr(&self) -> &Iclr {
        &self.iclr
    }
    #[doc = "0x78e0 - Event Mode"]
    #[inline(always)]
    pub const fn evt_mode(&self) -> &EvtMode {
        &self.evt_mode
    }
    #[doc = "0x78fc - Module Description"]
    #[inline(always)]
    pub const fn desc(&self) -> &Desc {
        &self.desc
    }
    #[doc = "0x7900 - MCAN module clock enable"]
    #[inline(always)]
    pub const fn mcanss_clken(&self) -> &McanssClken {
        &self.mcanss_clken
    }
    #[doc = "0x7904 - Clock divider"]
    #[inline(always)]
    pub const fn mcanss_clkdiv(&self) -> &McanssClkdiv {
        &self.mcanss_clkdiv
    }
    #[doc = "0x7908 - MCAN-SS clock stop control register"]
    #[inline(always)]
    pub const fn mcanss_clkctl(&self) -> &McanssClkctl {
        &self.mcanss_clkctl
    }
    #[doc = "0x790c - MCANSS clock stop status register"]
    #[inline(always)]
    pub const fn mcanss_clksts(&self) -> &McanssClksts {
        &self.mcanss_clksts
    }
}
#[doc = "PWREN (rw) register accessor: Power enable\n\nYou can [`read`](crate::Reg::read) this register and get [`pwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwren`]
module"]
#[doc(alias = "PWREN")]
pub type Pwren = crate::Reg<pwren::PwrenSpec>;
#[doc = "Power enable"]
pub mod pwren;
#[doc = "RSTCTL (w) register accessor: Reset Control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstctl`]
module"]
#[doc(alias = "RSTCTL")]
pub type Rstctl = crate::Reg<rstctl::RstctlSpec>;
#[doc = "Reset Control"]
pub mod rstctl;
#[doc = "STAT (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "Status Register"]
pub mod stat;
#[doc = "MCAN_CREL (r) register accessor: MCAN Core Release Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_crel::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_crel`]
module"]
#[doc(alias = "MCAN_CREL")]
pub type McanCrel = crate::Reg<mcan_crel::McanCrelSpec>;
#[doc = "MCAN Core Release Register"]
pub mod mcan_crel;
#[doc = "MCAN_ENDN (r) register accessor: MCAN Endian Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_endn::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_endn`]
module"]
#[doc(alias = "MCAN_ENDN")]
pub type McanEndn = crate::Reg<mcan_endn::McanEndnSpec>;
#[doc = "MCAN Endian Register"]
pub mod mcan_endn;
#[doc = "MCAN_DBTP (rw) register accessor: MCAN Data Bit Timing and Prescaler Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_dbtp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_dbtp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_dbtp`]
module"]
#[doc(alias = "MCAN_DBTP")]
pub type McanDbtp = crate::Reg<mcan_dbtp::McanDbtpSpec>;
#[doc = "MCAN Data Bit Timing and Prescaler Register"]
pub mod mcan_dbtp;
#[doc = "MCAN_TEST (rw) register accessor: MCAN Test Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_test::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_test::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_test`]
module"]
#[doc(alias = "MCAN_TEST")]
pub type McanTest = crate::Reg<mcan_test::McanTestSpec>;
#[doc = "MCAN Test Register"]
pub mod mcan_test;
#[doc = "MCAN_RWD (rw) register accessor: MCAN RAM Watchdog\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_rwd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_rwd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_rwd`]
module"]
#[doc(alias = "MCAN_RWD")]
pub type McanRwd = crate::Reg<mcan_rwd::McanRwdSpec>;
#[doc = "MCAN RAM Watchdog"]
pub mod mcan_rwd;
#[doc = "MCAN_CCCR (rw) register accessor: MCAN CC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_cccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_cccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_cccr`]
module"]
#[doc(alias = "MCAN_CCCR")]
pub type McanCccr = crate::Reg<mcan_cccr::McanCccrSpec>;
#[doc = "MCAN CC Control Register"]
pub mod mcan_cccr;
#[doc = "MCAN_NBTP (rw) register accessor: MCAN Nominal Bit Timing and Prescaler Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_nbtp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_nbtp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_nbtp`]
module"]
#[doc(alias = "MCAN_NBTP")]
pub type McanNbtp = crate::Reg<mcan_nbtp::McanNbtpSpec>;
#[doc = "MCAN Nominal Bit Timing and Prescaler Register"]
pub mod mcan_nbtp;
#[doc = "MCAN_TSCC (rw) register accessor: MCAN Timestamp Counter Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_tscc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_tscc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_tscc`]
module"]
#[doc(alias = "MCAN_TSCC")]
pub type McanTscc = crate::Reg<mcan_tscc::McanTsccSpec>;
#[doc = "MCAN Timestamp Counter Configuration"]
pub mod mcan_tscc;
#[doc = "MCAN_TSCV (rw) register accessor: MCAN Timestamp Counter Value\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_tscv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_tscv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_tscv`]
module"]
#[doc(alias = "MCAN_TSCV")]
pub type McanTscv = crate::Reg<mcan_tscv::McanTscvSpec>;
#[doc = "MCAN Timestamp Counter Value"]
pub mod mcan_tscv;
#[doc = "MCAN_TOCC (rw) register accessor: MCAN Timeout Counter Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_tocc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_tocc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_tocc`]
module"]
#[doc(alias = "MCAN_TOCC")]
pub type McanTocc = crate::Reg<mcan_tocc::McanToccSpec>;
#[doc = "MCAN Timeout Counter Configuration"]
pub mod mcan_tocc;
#[doc = "MCAN_TOCV (rw) register accessor: MCAN Timeout Counter Value\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_tocv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_tocv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_tocv`]
module"]
#[doc(alias = "MCAN_TOCV")]
pub type McanTocv = crate::Reg<mcan_tocv::McanTocvSpec>;
#[doc = "MCAN Timeout Counter Value"]
pub mod mcan_tocv;
#[doc = "MCAN_ECR (r) register accessor: MCAN Error Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_ecr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_ecr`]
module"]
#[doc(alias = "MCAN_ECR")]
pub type McanEcr = crate::Reg<mcan_ecr::McanEcrSpec>;
#[doc = "MCAN Error Counter Register"]
pub mod mcan_ecr;
#[doc = "MCAN_PSR (r) register accessor: MCAN Protocol Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_psr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_psr`]
module"]
#[doc(alias = "MCAN_PSR")]
pub type McanPsr = crate::Reg<mcan_psr::McanPsrSpec>;
#[doc = "MCAN Protocol Status Register"]
pub mod mcan_psr;
#[doc = "MCAN_TDCR (rw) register accessor: MCAN Transmitter Delay Compensation Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_tdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_tdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_tdcr`]
module"]
#[doc(alias = "MCAN_TDCR")]
pub type McanTdcr = crate::Reg<mcan_tdcr::McanTdcrSpec>;
#[doc = "MCAN Transmitter Delay Compensation Register"]
pub mod mcan_tdcr;
#[doc = "MCAN_IR (rw) register accessor: MCAN Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_ir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_ir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_ir`]
module"]
#[doc(alias = "MCAN_IR")]
pub type McanIr = crate::Reg<mcan_ir::McanIrSpec>;
#[doc = "MCAN Interrupt Register"]
pub mod mcan_ir;
#[doc = "MCAN_IE (rw) register accessor: MCAN Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_ie`]
module"]
#[doc(alias = "MCAN_IE")]
pub type McanIe = crate::Reg<mcan_ie::McanIeSpec>;
#[doc = "MCAN Interrupt Enable"]
pub mod mcan_ie;
#[doc = "MCAN_ILS (rw) register accessor: MCAN Interrupt Line Select\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_ils::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_ils::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_ils`]
module"]
#[doc(alias = "MCAN_ILS")]
pub type McanIls = crate::Reg<mcan_ils::McanIlsSpec>;
#[doc = "MCAN Interrupt Line Select"]
pub mod mcan_ils;
#[doc = "MCAN_ILE (rw) register accessor: MCAN Interrupt Line Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_ile::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_ile::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_ile`]
module"]
#[doc(alias = "MCAN_ILE")]
pub type McanIle = crate::Reg<mcan_ile::McanIleSpec>;
#[doc = "MCAN Interrupt Line Enable"]
pub mod mcan_ile;
#[doc = "MCAN_GFC (rw) register accessor: MCAN Global Filter Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_gfc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_gfc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_gfc`]
module"]
#[doc(alias = "MCAN_GFC")]
pub type McanGfc = crate::Reg<mcan_gfc::McanGfcSpec>;
#[doc = "MCAN Global Filter Configuration"]
pub mod mcan_gfc;
#[doc = "MCAN_SIDFC (rw) register accessor: MCAN Standard ID Filter Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_sidfc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_sidfc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_sidfc`]
module"]
#[doc(alias = "MCAN_SIDFC")]
pub type McanSidfc = crate::Reg<mcan_sidfc::McanSidfcSpec>;
#[doc = "MCAN Standard ID Filter Configuration"]
pub mod mcan_sidfc;
#[doc = "MCAN_XIDFC (rw) register accessor: MCAN Extended ID Filter Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_xidfc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_xidfc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_xidfc`]
module"]
#[doc(alias = "MCAN_XIDFC")]
pub type McanXidfc = crate::Reg<mcan_xidfc::McanXidfcSpec>;
#[doc = "MCAN Extended ID Filter Configuration"]
pub mod mcan_xidfc;
#[doc = "MCAN_XIDAM (rw) register accessor: MCAN Extended ID and Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_xidam::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_xidam::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_xidam`]
module"]
#[doc(alias = "MCAN_XIDAM")]
pub type McanXidam = crate::Reg<mcan_xidam::McanXidamSpec>;
#[doc = "MCAN Extended ID and Mask"]
pub mod mcan_xidam;
#[doc = "MCAN_HPMS (r) register accessor: MCAN High Priority Message Status\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_hpms::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_hpms`]
module"]
#[doc(alias = "MCAN_HPMS")]
pub type McanHpms = crate::Reg<mcan_hpms::McanHpmsSpec>;
#[doc = "MCAN High Priority Message Status"]
pub mod mcan_hpms;
#[doc = "MCAN_NDAT1 (rw) register accessor: MCAN New Data 1\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_ndat1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_ndat1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_ndat1`]
module"]
#[doc(alias = "MCAN_NDAT1")]
pub type McanNdat1 = crate::Reg<mcan_ndat1::McanNdat1Spec>;
#[doc = "MCAN New Data 1"]
pub mod mcan_ndat1;
#[doc = "MCAN_NDAT2 (rw) register accessor: MCAN New Data 2\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_ndat2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_ndat2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_ndat2`]
module"]
#[doc(alias = "MCAN_NDAT2")]
pub type McanNdat2 = crate::Reg<mcan_ndat2::McanNdat2Spec>;
#[doc = "MCAN New Data 2"]
pub mod mcan_ndat2;
#[doc = "MCAN_RXF0C (rw) register accessor: MCAN Rx FIFO 0 Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_rxf0c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_rxf0c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_rxf0c`]
module"]
#[doc(alias = "MCAN_RXF0C")]
pub type McanRxf0c = crate::Reg<mcan_rxf0c::McanRxf0cSpec>;
#[doc = "MCAN Rx FIFO 0 Configuration"]
pub mod mcan_rxf0c;
#[doc = "MCAN_RXF0S (r) register accessor: MCAN Rx FIFO 0 Status\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_rxf0s::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_rxf0s`]
module"]
#[doc(alias = "MCAN_RXF0S")]
pub type McanRxf0s = crate::Reg<mcan_rxf0s::McanRxf0sSpec>;
#[doc = "MCAN Rx FIFO 0 Status"]
pub mod mcan_rxf0s;
#[doc = "MCAN_RXF0A (rw) register accessor: MCAN Rx FIFO 0 Acknowledge\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_rxf0a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_rxf0a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_rxf0a`]
module"]
#[doc(alias = "MCAN_RXF0A")]
pub type McanRxf0a = crate::Reg<mcan_rxf0a::McanRxf0aSpec>;
#[doc = "MCAN Rx FIFO 0 Acknowledge"]
pub mod mcan_rxf0a;
#[doc = "MCAN_RXBC (rw) register accessor: MCAN Rx Buffer Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_rxbc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_rxbc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_rxbc`]
module"]
#[doc(alias = "MCAN_RXBC")]
pub type McanRxbc = crate::Reg<mcan_rxbc::McanRxbcSpec>;
#[doc = "MCAN Rx Buffer Configuration"]
pub mod mcan_rxbc;
#[doc = "MCAN_RXF1C (rw) register accessor: MCAN Rx FIFO 1 Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_rxf1c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_rxf1c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_rxf1c`]
module"]
#[doc(alias = "MCAN_RXF1C")]
pub type McanRxf1c = crate::Reg<mcan_rxf1c::McanRxf1cSpec>;
#[doc = "MCAN Rx FIFO 1 Configuration"]
pub mod mcan_rxf1c;
#[doc = "MCAN_RXF1S (r) register accessor: MCAN Rx FIFO 1 Status\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_rxf1s::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_rxf1s`]
module"]
#[doc(alias = "MCAN_RXF1S")]
pub type McanRxf1s = crate::Reg<mcan_rxf1s::McanRxf1sSpec>;
#[doc = "MCAN Rx FIFO 1 Status"]
pub mod mcan_rxf1s;
#[doc = "MCAN_RXF1A (rw) register accessor: MCAN Rx FIFO 1 Acknowledge\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_rxf1a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_rxf1a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_rxf1a`]
module"]
#[doc(alias = "MCAN_RXF1A")]
pub type McanRxf1a = crate::Reg<mcan_rxf1a::McanRxf1aSpec>;
#[doc = "MCAN Rx FIFO 1 Acknowledge"]
pub mod mcan_rxf1a;
#[doc = "MCAN_RXESC (rw) register accessor: MCAN Rx Buffer / FIFO Element Size Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_rxesc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_rxesc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_rxesc`]
module"]
#[doc(alias = "MCAN_RXESC")]
pub type McanRxesc = crate::Reg<mcan_rxesc::McanRxescSpec>;
#[doc = "MCAN Rx Buffer / FIFO Element Size Configuration"]
pub mod mcan_rxesc;
#[doc = "MCAN_TXBC (rw) register accessor: MCAN Tx Buffer Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_txbc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_txbc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_txbc`]
module"]
#[doc(alias = "MCAN_TXBC")]
pub type McanTxbc = crate::Reg<mcan_txbc::McanTxbcSpec>;
#[doc = "MCAN Tx Buffer Configuration"]
pub mod mcan_txbc;
#[doc = "MCAN_TXFQS (r) register accessor: MCAN Tx FIFO / Queue Status\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_txfqs::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_txfqs`]
module"]
#[doc(alias = "MCAN_TXFQS")]
pub type McanTxfqs = crate::Reg<mcan_txfqs::McanTxfqsSpec>;
#[doc = "MCAN Tx FIFO / Queue Status"]
pub mod mcan_txfqs;
#[doc = "MCAN_TXESC (rw) register accessor: MCAN Tx Buffer Element Size Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_txesc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_txesc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_txesc`]
module"]
#[doc(alias = "MCAN_TXESC")]
pub type McanTxesc = crate::Reg<mcan_txesc::McanTxescSpec>;
#[doc = "MCAN Tx Buffer Element Size Configuration"]
pub mod mcan_txesc;
#[doc = "MCAN_TXBRP (r) register accessor: MCAN Tx Buffer Request Pending\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_txbrp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_txbrp`]
module"]
#[doc(alias = "MCAN_TXBRP")]
pub type McanTxbrp = crate::Reg<mcan_txbrp::McanTxbrpSpec>;
#[doc = "MCAN Tx Buffer Request Pending"]
pub mod mcan_txbrp;
#[doc = "MCAN_TXBAR (rw) register accessor: MCAN Tx Buffer Add Request\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_txbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_txbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_txbar`]
module"]
#[doc(alias = "MCAN_TXBAR")]
pub type McanTxbar = crate::Reg<mcan_txbar::McanTxbarSpec>;
#[doc = "MCAN Tx Buffer Add Request"]
pub mod mcan_txbar;
#[doc = "MCAN_TXBCR (rw) register accessor: MCAN Tx Buffer Cancellation Request\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_txbcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_txbcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_txbcr`]
module"]
#[doc(alias = "MCAN_TXBCR")]
pub type McanTxbcr = crate::Reg<mcan_txbcr::McanTxbcrSpec>;
#[doc = "MCAN Tx Buffer Cancellation Request"]
pub mod mcan_txbcr;
#[doc = "MCAN_TXBTO (r) register accessor: MCAN Tx Buffer Transmission Occurred\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_txbto::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_txbto`]
module"]
#[doc(alias = "MCAN_TXBTO")]
pub type McanTxbto = crate::Reg<mcan_txbto::McanTxbtoSpec>;
#[doc = "MCAN Tx Buffer Transmission Occurred"]
pub mod mcan_txbto;
#[doc = "MCAN_TXBCF (r) register accessor: MCAN Tx Buffer Cancellation Finished\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_txbcf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_txbcf`]
module"]
#[doc(alias = "MCAN_TXBCF")]
pub type McanTxbcf = crate::Reg<mcan_txbcf::McanTxbcfSpec>;
#[doc = "MCAN Tx Buffer Cancellation Finished"]
pub mod mcan_txbcf;
#[doc = "MCAN_TXBTIE (rw) register accessor: MCAN Tx Buffer Transmission Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_txbtie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_txbtie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_txbtie`]
module"]
#[doc(alias = "MCAN_TXBTIE")]
pub type McanTxbtie = crate::Reg<mcan_txbtie::McanTxbtieSpec>;
#[doc = "MCAN Tx Buffer Transmission Interrupt Enable"]
pub mod mcan_txbtie;
#[doc = "MCAN_TXBCIE (rw) register accessor: MCAN Tx Buffer Cancellation Finished Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_txbcie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_txbcie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_txbcie`]
module"]
#[doc(alias = "MCAN_TXBCIE")]
pub type McanTxbcie = crate::Reg<mcan_txbcie::McanTxbcieSpec>;
#[doc = "MCAN Tx Buffer Cancellation Finished Interrupt Enable"]
pub mod mcan_txbcie;
#[doc = "MCAN_TXEFC (rw) register accessor: MCAN Tx Event FIFO Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_txefc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_txefc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_txefc`]
module"]
#[doc(alias = "MCAN_TXEFC")]
pub type McanTxefc = crate::Reg<mcan_txefc::McanTxefcSpec>;
#[doc = "MCAN Tx Event FIFO Configuration"]
pub mod mcan_txefc;
#[doc = "MCAN_TXEFS (r) register accessor: MCAN Tx Event FIFO Status\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_txefs::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_txefs`]
module"]
#[doc(alias = "MCAN_TXEFS")]
pub type McanTxefs = crate::Reg<mcan_txefs::McanTxefsSpec>;
#[doc = "MCAN Tx Event FIFO Status"]
pub mod mcan_txefs;
#[doc = "MCAN_TXEFA (rw) register accessor: MCAN Tx Event FIFO Acknowledge\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_txefa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_txefa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcan_txefa`]
module"]
#[doc(alias = "MCAN_TXEFA")]
pub type McanTxefa = crate::Reg<mcan_txefa::McanTxefaSpec>;
#[doc = "MCAN Tx Event FIFO Acknowledge"]
pub mod mcan_txefa;
#[doc = "MCANSS_PID (r) register accessor: MCAN Subsystem Revision Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanss_pid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcanss_pid`]
module"]
#[doc(alias = "MCANSS_PID")]
pub type McanssPid = crate::Reg<mcanss_pid::McanssPidSpec>;
#[doc = "MCAN Subsystem Revision Register"]
pub mod mcanss_pid;
#[doc = "MCANSS_CTRL (rw) register accessor: MCAN Subsystem Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanss_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcanss_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcanss_ctrl`]
module"]
#[doc(alias = "MCANSS_CTRL")]
pub type McanssCtrl = crate::Reg<mcanss_ctrl::McanssCtrlSpec>;
#[doc = "MCAN Subsystem Control Register"]
pub mod mcanss_ctrl;
#[doc = "MCANSS_STAT (r) register accessor: MCAN Subsystem Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanss_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcanss_stat`]
module"]
#[doc(alias = "MCANSS_STAT")]
pub type McanssStat = crate::Reg<mcanss_stat::McanssStatSpec>;
#[doc = "MCAN Subsystem Status Register"]
pub mod mcanss_stat;
#[doc = "MCANSS_ICS (rw) register accessor: MCAN Subsystem Interrupt Clear Shadow Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanss_ics::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcanss_ics::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcanss_ics`]
module"]
#[doc(alias = "MCANSS_ICS")]
pub type McanssIcs = crate::Reg<mcanss_ics::McanssIcsSpec>;
#[doc = "MCAN Subsystem Interrupt Clear Shadow Register"]
pub mod mcanss_ics;
#[doc = "MCANSS_IRS (rw) register accessor: MCAN Subsystem Interrupt Raw Satus Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanss_irs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcanss_irs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcanss_irs`]
module"]
#[doc(alias = "MCANSS_IRS")]
pub type McanssIrs = crate::Reg<mcanss_irs::McanssIrsSpec>;
#[doc = "MCAN Subsystem Interrupt Raw Satus Register"]
pub mod mcanss_irs;
#[doc = "MCANSS_IECS (rw) register accessor: MCAN Subsystem Interrupt Enable Clear Shadow Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanss_iecs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcanss_iecs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcanss_iecs`]
module"]
#[doc(alias = "MCANSS_IECS")]
pub type McanssIecs = crate::Reg<mcanss_iecs::McanssIecsSpec>;
#[doc = "MCAN Subsystem Interrupt Enable Clear Shadow Register"]
pub mod mcanss_iecs;
#[doc = "MCANSS_IE (rw) register accessor: MCAN Subsystem Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanss_ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcanss_ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcanss_ie`]
module"]
#[doc(alias = "MCANSS_IE")]
pub type McanssIe = crate::Reg<mcanss_ie::McanssIeSpec>;
#[doc = "MCAN Subsystem Interrupt Enable Register"]
pub mod mcanss_ie;
#[doc = "MCANSS_IES (r) register accessor: MCAN Subsystem Interrupt Enable Status\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanss_ies::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcanss_ies`]
module"]
#[doc(alias = "MCANSS_IES")]
pub type McanssIes = crate::Reg<mcanss_ies::McanssIesSpec>;
#[doc = "MCAN Subsystem Interrupt Enable Status"]
pub mod mcanss_ies;
#[doc = "MCANSS_EOI (rw) register accessor: MCAN Subsystem End of Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanss_eoi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcanss_eoi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcanss_eoi`]
module"]
#[doc(alias = "MCANSS_EOI")]
pub type McanssEoi = crate::Reg<mcanss_eoi::McanssEoiSpec>;
#[doc = "MCAN Subsystem End of Interrupt"]
pub mod mcanss_eoi;
#[doc = "MCANSS_EXT_TS_PRESCALER (rw) register accessor: MCAN Subsystem External Timestamp Prescaler 0\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanss_ext_ts_prescaler::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcanss_ext_ts_prescaler::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcanss_ext_ts_prescaler`]
module"]
#[doc(alias = "MCANSS_EXT_TS_PRESCALER")]
pub type McanssExtTsPrescaler = crate::Reg<mcanss_ext_ts_prescaler::McanssExtTsPrescalerSpec>;
#[doc = "MCAN Subsystem External Timestamp Prescaler 0"]
pub mod mcanss_ext_ts_prescaler;
#[doc = "MCANSS_EXT_TS_UNSERVICED_INTR_CNTR (r) register accessor: MCAN Subsystem External Timestamp Unserviced Interrupts Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanss_ext_ts_unserviced_intr_cntr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcanss_ext_ts_unserviced_intr_cntr`]
module"]
#[doc(alias = "MCANSS_EXT_TS_UNSERVICED_INTR_CNTR")]
pub type McanssExtTsUnservicedIntrCntr =
    crate::Reg<mcanss_ext_ts_unserviced_intr_cntr::McanssExtTsUnservicedIntrCntrSpec>;
#[doc = "MCAN Subsystem External Timestamp Unserviced Interrupts Counter"]
pub mod mcanss_ext_ts_unserviced_intr_cntr;
#[doc = "MCANERR_REV (r) register accessor: MCAN Error Aggregator Revision Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanerr_rev::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcanerr_rev`]
module"]
#[doc(alias = "MCANERR_REV")]
pub type McanerrRev = crate::Reg<mcanerr_rev::McanerrRevSpec>;
#[doc = "MCAN Error Aggregator Revision Register"]
pub mod mcanerr_rev;
#[doc = "MCANERR_VECTOR (rw) register accessor: MCAN ECC Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanerr_vector::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcanerr_vector::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcanerr_vector`]
module"]
#[doc(alias = "MCANERR_VECTOR")]
pub type McanerrVector = crate::Reg<mcanerr_vector::McanerrVectorSpec>;
#[doc = "MCAN ECC Vector Register"]
pub mod mcanerr_vector;
#[doc = "MCANERR_STAT (r) register accessor: MCAN Error Misc Status\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanerr_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcanerr_stat`]
module"]
#[doc(alias = "MCANERR_STAT")]
pub type McanerrStat = crate::Reg<mcanerr_stat::McanerrStatSpec>;
#[doc = "MCAN Error Misc Status"]
pub mod mcanerr_stat;
#[doc = "MCANERR_WRAP_REV (r) register accessor: MCAN ECC Wrapper Revision Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanerr_wrap_rev::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcanerr_wrap_rev`]
module"]
#[doc(alias = "MCANERR_WRAP_REV")]
pub type McanerrWrapRev = crate::Reg<mcanerr_wrap_rev::McanerrWrapRevSpec>;
#[doc = "MCAN ECC Wrapper Revision Register"]
pub mod mcanerr_wrap_rev;
#[doc = "MCANERR_CTRL (rw) register accessor: MCAN ECC Control\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanerr_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcanerr_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcanerr_ctrl`]
module"]
#[doc(alias = "MCANERR_CTRL")]
pub type McanerrCtrl = crate::Reg<mcanerr_ctrl::McanerrCtrlSpec>;
#[doc = "MCAN ECC Control"]
pub mod mcanerr_ctrl;
#[doc = "MCANERR_ERR_CTRL1 (rw) register accessor: MCAN ECC Error Control 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanerr_err_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcanerr_err_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcanerr_err_ctrl1`]
module"]
#[doc(alias = "MCANERR_ERR_CTRL1")]
pub type McanerrErrCtrl1 = crate::Reg<mcanerr_err_ctrl1::McanerrErrCtrl1Spec>;
#[doc = "MCAN ECC Error Control 1 Register"]
pub mod mcanerr_err_ctrl1;
#[doc = "MCANERR_ERR_CTRL2 (rw) register accessor: MCAN ECC Error Control 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanerr_err_ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcanerr_err_ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcanerr_err_ctrl2`]
module"]
#[doc(alias = "MCANERR_ERR_CTRL2")]
pub type McanerrErrCtrl2 = crate::Reg<mcanerr_err_ctrl2::McanerrErrCtrl2Spec>;
#[doc = "MCAN ECC Error Control 2 Register"]
pub mod mcanerr_err_ctrl2;
#[doc = "MCANERR_ERR_STAT1 (rw) register accessor: MCAN ECC Error Status 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanerr_err_stat1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcanerr_err_stat1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcanerr_err_stat1`]
module"]
#[doc(alias = "MCANERR_ERR_STAT1")]
pub type McanerrErrStat1 = crate::Reg<mcanerr_err_stat1::McanerrErrStat1Spec>;
#[doc = "MCAN ECC Error Status 1 Register"]
pub mod mcanerr_err_stat1;
#[doc = "MCANERR_ERR_STAT2 (r) register accessor: MCAN ECC Error Status 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanerr_err_stat2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcanerr_err_stat2`]
module"]
#[doc(alias = "MCANERR_ERR_STAT2")]
pub type McanerrErrStat2 = crate::Reg<mcanerr_err_stat2::McanerrErrStat2Spec>;
#[doc = "MCAN ECC Error Status 2 Register"]
pub mod mcanerr_err_stat2;
#[doc = "MCANERR_ERR_STAT3 (rw) register accessor: MCAN ECC Error Status 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanerr_err_stat3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcanerr_err_stat3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcanerr_err_stat3`]
module"]
#[doc(alias = "MCANERR_ERR_STAT3")]
pub type McanerrErrStat3 = crate::Reg<mcanerr_err_stat3::McanerrErrStat3Spec>;
#[doc = "MCAN ECC Error Status 3 Register"]
pub mod mcanerr_err_stat3;
#[doc = "MCANERR_SEC_EOI (rw) register accessor: MCAN Single Error Corrected End of Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanerr_sec_eoi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcanerr_sec_eoi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcanerr_sec_eoi`]
module"]
#[doc(alias = "MCANERR_SEC_EOI")]
pub type McanerrSecEoi = crate::Reg<mcanerr_sec_eoi::McanerrSecEoiSpec>;
#[doc = "MCAN Single Error Corrected End of Interrupt Register"]
pub mod mcanerr_sec_eoi;
#[doc = "MCANERR_SEC_STATUS (rw) register accessor: MCAN Single Error Corrected Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanerr_sec_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcanerr_sec_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcanerr_sec_status`]
module"]
#[doc(alias = "MCANERR_SEC_STATUS")]
pub type McanerrSecStatus = crate::Reg<mcanerr_sec_status::McanerrSecStatusSpec>;
#[doc = "MCAN Single Error Corrected Interrupt Status Register"]
pub mod mcanerr_sec_status;
#[doc = "MCANERR_SEC_ENABLE_SET (rw) register accessor: MCAN Single Error Corrected Interrupt Enable Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanerr_sec_enable_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcanerr_sec_enable_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcanerr_sec_enable_set`]
module"]
#[doc(alias = "MCANERR_SEC_ENABLE_SET")]
pub type McanerrSecEnableSet = crate::Reg<mcanerr_sec_enable_set::McanerrSecEnableSetSpec>;
#[doc = "MCAN Single Error Corrected Interrupt Enable Set Register"]
pub mod mcanerr_sec_enable_set;
#[doc = "MCANERR_SEC_ENABLE_CLR (rw) register accessor: MCAN Single Error Corrected Interrupt Enable Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanerr_sec_enable_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcanerr_sec_enable_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcanerr_sec_enable_clr`]
module"]
#[doc(alias = "MCANERR_SEC_ENABLE_CLR")]
pub type McanerrSecEnableClr = crate::Reg<mcanerr_sec_enable_clr::McanerrSecEnableClrSpec>;
#[doc = "MCAN Single Error Corrected Interrupt Enable Clear Register"]
pub mod mcanerr_sec_enable_clr;
#[doc = "MCANERR_DED_EOI (rw) register accessor: MCAN Double Error Detected End of Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanerr_ded_eoi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcanerr_ded_eoi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcanerr_ded_eoi`]
module"]
#[doc(alias = "MCANERR_DED_EOI")]
pub type McanerrDedEoi = crate::Reg<mcanerr_ded_eoi::McanerrDedEoiSpec>;
#[doc = "MCAN Double Error Detected End of Interrupt Register"]
pub mod mcanerr_ded_eoi;
#[doc = "MCANERR_DED_STATUS (rw) register accessor: MCAN Double Error Detected Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanerr_ded_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcanerr_ded_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcanerr_ded_status`]
module"]
#[doc(alias = "MCANERR_DED_STATUS")]
pub type McanerrDedStatus = crate::Reg<mcanerr_ded_status::McanerrDedStatusSpec>;
#[doc = "MCAN Double Error Detected Interrupt Status Register"]
pub mod mcanerr_ded_status;
#[doc = "MCANERR_DED_ENABLE_SET (rw) register accessor: MCAN Double Error Detected Interrupt Enable Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanerr_ded_enable_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcanerr_ded_enable_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcanerr_ded_enable_set`]
module"]
#[doc(alias = "MCANERR_DED_ENABLE_SET")]
pub type McanerrDedEnableSet = crate::Reg<mcanerr_ded_enable_set::McanerrDedEnableSetSpec>;
#[doc = "MCAN Double Error Detected Interrupt Enable Set Register"]
pub mod mcanerr_ded_enable_set;
#[doc = "MCANERR_DED_ENABLE_CLR (rw) register accessor: MCAN Double Error Detected Interrupt Enable Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanerr_ded_enable_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcanerr_ded_enable_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcanerr_ded_enable_clr`]
module"]
#[doc(alias = "MCANERR_DED_ENABLE_CLR")]
pub type McanerrDedEnableClr = crate::Reg<mcanerr_ded_enable_clr::McanerrDedEnableClrSpec>;
#[doc = "MCAN Double Error Detected Interrupt Enable Clear Register"]
pub mod mcanerr_ded_enable_clr;
#[doc = "MCANERR_AGGR_ENABLE_SET (rw) register accessor: MCAN Error Aggregator Enable Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanerr_aggr_enable_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcanerr_aggr_enable_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcanerr_aggr_enable_set`]
module"]
#[doc(alias = "MCANERR_AGGR_ENABLE_SET")]
pub type McanerrAggrEnableSet = crate::Reg<mcanerr_aggr_enable_set::McanerrAggrEnableSetSpec>;
#[doc = "MCAN Error Aggregator Enable Set Register"]
pub mod mcanerr_aggr_enable_set;
#[doc = "MCANERR_AGGR_ENABLE_CLR (rw) register accessor: MCAN Error Aggregator Enable Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanerr_aggr_enable_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcanerr_aggr_enable_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcanerr_aggr_enable_clr`]
module"]
#[doc(alias = "MCANERR_AGGR_ENABLE_CLR")]
pub type McanerrAggrEnableClr = crate::Reg<mcanerr_aggr_enable_clr::McanerrAggrEnableClrSpec>;
#[doc = "MCAN Error Aggregator Enable Clear Register"]
pub mod mcanerr_aggr_enable_clr;
#[doc = "MCANERR_AGGR_STATUS_SET (rw) register accessor: MCAN Error Aggregator Status Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanerr_aggr_status_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcanerr_aggr_status_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcanerr_aggr_status_set`]
module"]
#[doc(alias = "MCANERR_AGGR_STATUS_SET")]
pub type McanerrAggrStatusSet = crate::Reg<mcanerr_aggr_status_set::McanerrAggrStatusSetSpec>;
#[doc = "MCAN Error Aggregator Status Set Register"]
pub mod mcanerr_aggr_status_set;
#[doc = "MCANERR_AGGR_STATUS_CLR (rw) register accessor: MCAN Error Aggregator Status Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanerr_aggr_status_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcanerr_aggr_status_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcanerr_aggr_status_clr`]
module"]
#[doc(alias = "MCANERR_AGGR_STATUS_CLR")]
pub type McanerrAggrStatusClr = crate::Reg<mcanerr_aggr_status_clr::McanerrAggrStatusClrSpec>;
#[doc = "MCAN Error Aggregator Status Clear Register"]
pub mod mcanerr_aggr_status_clr;
#[doc = "IIDX (r) register accessor: Interrupt Index Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iidx`]
module"]
#[doc(alias = "IIDX")]
pub type Iidx = crate::Reg<iidx::IidxSpec>;
#[doc = "Interrupt Index Register"]
pub mod iidx;
#[doc = "IMASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`imask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imask`]
module"]
#[doc(alias = "IMASK")]
pub type Imask = crate::Reg<imask::ImaskSpec>;
#[doc = "Interrupt mask"]
pub mod imask;
#[doc = "RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ris`]
module"]
#[doc(alias = "RIS")]
pub type Ris = crate::Reg<ris::RisSpec>;
#[doc = "Raw interrupt status"]
pub mod ris;
#[doc = "MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mis`]
module"]
#[doc(alias = "MIS")]
pub type Mis = crate::Reg<mis::MisSpec>;
#[doc = "Masked interrupt status"]
pub mod mis;
#[doc = "ISET (w) register accessor: Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iset`]
module"]
#[doc(alias = "ISET")]
pub type Iset = crate::Reg<iset::IsetSpec>;
#[doc = "Interrupt set"]
pub mod iset;
#[doc = "ICLR (w) register accessor: Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iclr`]
module"]
#[doc(alias = "ICLR")]
pub type Iclr = crate::Reg<iclr::IclrSpec>;
#[doc = "Interrupt clear"]
pub mod iclr;
#[doc = "EVT_MODE (rw) register accessor: Event Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`evt_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_mode`]
module"]
#[doc(alias = "EVT_MODE")]
pub type EvtMode = crate::Reg<evt_mode::EvtModeSpec>;
#[doc = "Event Mode"]
pub mod evt_mode;
#[doc = "DESC (r) register accessor: Module Description\n\nYou can [`read`](crate::Reg::read) this register and get [`desc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@desc`]
module"]
#[doc(alias = "DESC")]
pub type Desc = crate::Reg<desc::DescSpec>;
#[doc = "Module Description"]
pub mod desc;
#[doc = "MCANSS_CLKEN (rw) register accessor: MCAN module clock enable\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanss_clken::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcanss_clken::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcanss_clken`]
module"]
#[doc(alias = "MCANSS_CLKEN")]
pub type McanssClken = crate::Reg<mcanss_clken::McanssClkenSpec>;
#[doc = "MCAN module clock enable"]
pub mod mcanss_clken;
#[doc = "MCANSS_CLKDIV (rw) register accessor: Clock divider\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanss_clkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcanss_clkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcanss_clkdiv`]
module"]
#[doc(alias = "MCANSS_CLKDIV")]
pub type McanssClkdiv = crate::Reg<mcanss_clkdiv::McanssClkdivSpec>;
#[doc = "Clock divider"]
pub mod mcanss_clkdiv;
#[doc = "MCANSS_CLKCTL (rw) register accessor: MCAN-SS clock stop control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanss_clkctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcanss_clkctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcanss_clkctl`]
module"]
#[doc(alias = "MCANSS_CLKCTL")]
pub type McanssClkctl = crate::Reg<mcanss_clkctl::McanssClkctlSpec>;
#[doc = "MCAN-SS clock stop control register"]
pub mod mcanss_clkctl;
#[doc = "MCANSS_CLKSTS (r) register accessor: MCANSS clock stop status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanss_clksts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcanss_clksts`]
module"]
#[doc(alias = "MCANSS_CLKSTS")]
pub type McanssClksts = crate::Reg<mcanss_clksts::McanssClkstsSpec>;
#[doc = "MCANSS clock stop status register"]
pub mod mcanss_clksts;
