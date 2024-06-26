use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

/// The data that is needed to interact with the SnP modules.
#[derive(Debug, Deserialize)]
pub struct Data {
    pub symbol: Rc<str>,
    pub security: Rc<str>,
    pub sector: Rc<str>,
    pub sub_sector: Rc<str>,
    pub headquarters_location: Rc<str>,
    pub date_added: NaiveDate,
    pub cik: Rc<str>,
    pub founded: Rc<str>,
}

/// The S&P 500 symbols
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
pub enum SnP {
    MMM,
    AOS,
    ABT,
    ABBV,
    ACN,
    ADBE,
    AMD,
    AES,
    AFL,
    A,
    APD,
    ABNB,
    AKAM,
    ALB,
    ARE,
    ALGN,
    ALLE,
    LNT,
    ALL,
    GOOGL,
    GOOG,
    MO,
    AMZN,
    AMCR,
    AEE,
    AAL,
    AEP,
    AXP,
    AIG,
    AMT,
    AWK,
    AMP,
    AME,
    AMGN,
    APH,
    ADI,
    ANSS,
    AON,
    APA,
    AAPL,
    AMAT,
    APTV,
    ACGL,
    ADM,
    ANET,
    AJG,
    AIZ,
    T,
    ATO,
    ADSK,
    ADP,
    AZO,
    AVB,
    AVY,
    AXON,
    BKR,
    BALL,
    BAC,
    BK,
    BBWI,
    BAX,
    BDX,
    BRK,
    BBY,
    BIO,
    TECH,
    BIIB,
    BLK,
    BX,
    BA,
    BKNG,
    BWA,
    BXP,
    BSX,
    BMY,
    AVGO,
    BR,
    BRO,
    BF,
    BLDR,
    BG,
    CDNS,
    CZR,
    CPT,
    CPB,
    COF,
    CAH,
    KMX,
    CCL,
    CARR,
    CTLT,
    CAT,
    CBOE,
    CBRE,
    CDW,
    CE,
    COR,
    CNC,
    CNP,
    CF,
    CHRW,
    CRL,
    SCHW,
    CHTR,
    CVX,
    CMG,
    CB,
    CHD,
    CI,
    CINF,
    CTAS,
    CSCO,
    C,
    CFG,
    CLX,
    CME,
    CMS,
    KO,
    CTSH,
    CL,
    CMCSA,
    CMA,
    CAG,
    COP,
    ED,
    STZ,
    CEG,
    COO,
    CPRT,
    GLW,
    CTVA,
    CSGP,
    COST,
    CTRA,
    CCI,
    CSX,
    CMI,
    CVS,
    DHR,
    DRI,
    DVA,
    DAY,
    DE,
    DAL,
    XRAY,
    DVN,
    DXCM,
    FANG,
    DLR,
    DFS,
    DG,
    DLTR,
    D,
    DPZ,
    DOV,
    DOW,
    DHI,
    DTE,
    DUK,
    DD,
    EMN,
    ETN,
    EBAY,
    ECL,
    EIX,
    EW,
    EA,
    ELV,
    LLY,
    EMR,
    ENPH,
    ETR,
    EOG,
    EPAM,
    EQT,
    EFX,
    EQIX,
    EQR,
    ESS,
    EL,
    ETSY,
    EG,
    EVRG,
    ES,
    EXC,
    EXPE,
    EXPD,
    EXR,
    XOM,
    FFIV,
    FDS,
    FICO,
    FAST,
    FRT,
    FDX,
    FIS,
    FITB,
    FSLR,
    FE,
    FI,
    FLT,
    FMC,
    F,
    FTNT,
    FTV,
    FOXA,
    FOX,
    BEN,
    FCX,
    GRMN,
    IT,
    GEHC,
    GEN,
    GNRC,
    GD,
    GE,
    GIS,
    GM,
    GPC,
    GILD,
    GPN,
    GL,
    GS,
    HAL,
    HIG,
    HAS,
    HCA,
    DOC,
    HSIC,
    HSY,
    HES,
    HPE,
    HLT,
    HOLX,
    HD,
    HON,
    HRL,
    HST,
    HWM,
    HPQ,
    HUBB,
    HUM,
    HBAN,
    HII,
    IBM,
    IEX,
    IDXX,
    ITW,
    ILMN,
    INCY,
    IR,
    PODD,
    INTC,
    ICE,
    IFF,
    IP,
    IPG,
    INTU,
    ISRG,
    IVZ,
    INVH,
    IQV,
    IRM,
    JBHT,
    JBL,
    JKHY,
    J,
    JNJ,
    JCI,
    JPM,
    JNPR,
    K,
    KVUE,
    KDP,
    KEY,
    KEYS,
    KMB,
    KIM,
    KMI,
    KLAC,
    KHC,
    KR,
    LHX,
    LH,
    LRCX,
    LW,
    LVS,
    LDOS,
    LEN,
    LIN,
    LYV,
    LKQ,
    LMT,
    L,
    LOW,
    LULU,
    LYB,
    MTB,
    MRO,
    MPC,
    MKTX,
    MAR,
    MMC,
    MLM,
    MAS,
    MA,
    MTCH,
    MKC,
    MCD,
    MCK,
    MDT,
    MRK,
    META,
    MET,
    MTD,
    MGM,
    MCHP,
    MU,
    MSFT,
    MAA,
    MRNA,
    MHK,
    MOH,
    TAP,
    MDLZ,
    MPWR,
    MNST,
    MCO,
    MS,
    MOS,
    MSI,
    MSCI,
    NDAQ,
    NTAP,
    NFLX,
    NEM,
    NWSA,
    NWS,
    NEE,
    NKE,
    NI,
    NDSN,
    NSC,
    NTRS,
    NOC,
    NCLH,
    NRG,
    NUE,
    NVDA,
    NVR,
    NXPI,
    ORLY,
    OXY,
    ODFL,
    OMC,
    ON,
    OKE,
    ORCL,
    OTIS,
    PCAR,
    PKG,
    PANW,
    PARA,
    PH,
    PAYX,
    PAYC,
    PYPL,
    PNR,
    PEP,
    PFE,
    PCG,
    PM,
    PSX,
    PNW,
    PXD,
    PNC,
    POOL,
    PPG,
    PPL,
    PFG,
    PG,
    PGR,
    PLD,
    PRU,
    PEG,
    PTC,
    PSA,
    PHM,
    QRVO,
    PWR,
    QCOM,
    DGX,
    RL,
    RJF,
    RTX,
    O,
    REG,
    REGN,
    RF,
    RSG,
    RMD,
    RVTY,
    RHI,
    ROK,
    ROL,
    ROP,
    ROST,
    RCL,
    SPGI,
    CRM,
    SBAC,
    SLB,
    STX,
    SRE,
    NOW,
    SHW,
    SPG,
    SWKS,
    SJM,
    SNA,
    SO,
    LUV,
    SWK,
    SBUX,
    STT,
    STLD,
    STE,
    SYK,
    SYF,
    SNPS,
    SYY,
    TMUS,
    TROW,
    TTWO,
    TPR,
    TRGP,
    TGT,
    TEL,
    TDY,
    TFX,
    TER,
    TSLA,
    TXN,
    TXT,
    TMO,
    TJX,
    TSCO,
    TT,
    TDG,
    TRV,
    TRMB,
    TFC,
    TYL,
    TSN,
    USB,
    UBER,
    UDR,
    ULTA,
    UNP,
    UAL,
    UPS,
    URI,
    UNH,
    UHS,
    VLO,
    VTR,
    VLTO,
    VRSN,
    VRSK,
    VZ,
    VRTX,
    VFC,
    VTRS,
    VICI,
    V,
    VMC,
    WRB,
    WAB,
    WBA,
    WMT,
    DIS,
    WBD,
    WM,
    WAT,
    WEC,
    WFC,
    WELL,
    WST,
    WDC,
    WRK,
    WY,
    WHR,
    WMB,
    WTW,
    GWW,
    WYNN,
    XEL,
    XYL,
    YUM,
    ZBRA,
    ZBH,
    ZION,
    ZTS,

    SEE,
    ALK,
    SEDG,
    OGN,
    ATVI,
    DXC,
    LNC,
    NWL,
    AAP,
    DISH,
    FRC,
    LUMN,
    SBNY,
    SIVB,
    VNO,
    ABMD,
    FBHS,
    MBC,
    TWTR,
    NLSN,
    CTXS,
    DRE,
    PVH,
    PENN,
    UA,
    UAA,
    IPGP,
    CERN,
    PBCT,
    INFO,
    XLNX,
    GPS,
    LEG,
    HBI,
    WU,
    KSU,
    PRGO,
    UNM,
    NOV,
    MXIM,
    ALXN,
    HFC,
    FLIR,
    VAR,
    FLS,
    SLG,
    XRX,
    VNT,
    FTI,
    CXO,
    TIF,
    AIV,
    NBL,
    ETFC,
    HRB,
    COTY,
    KSS,
    ADS,
    HOG,
    JWN,
    HP,
    CPRI,
    AGN,
    RT,
    M,
    RTN,
    ARNC,
    XEC,
    WCG,
    AMG,
    TRIP,
    MAC,
    STI,
    VIAB,
    CELG,
    NKTR,
    JEF,
    TSS,
    APC,
    FL,
    RHT,
    LLL,
    BMS,
    MAT,
    DWDP,
    FLR,
    BHF,
    GT,
    NFX,
    SCG,
    ESRX,
    COL,
    AET,
    SRCL,
    CA,
    EVHC,
    ANDV,
    XL,
    GGP,
    DPS,
    TWX,
    RRC,
    AYI,
    MON,
    NAVI,
    WYN,
    CSRA,
    SIG,
    PDCO,
    CHK,
    SNI,
    BCR,
    LVLT,
    SPLS,
    WFM,
    AN,
    RIG,
    BBBY,
    MUR,
    MNK,
    RAI,
    YHOO,
    TDC,
    R,
    MJN,
    TGNA,
    DNB,
    SWN,
    URBN,
    FTR,
    HAR,
    LLTC,
    ENDP,
    PBI,
    SE,
    STJ,
    OI,
    LM,
    AA,
    DO,
    HOT,
    EMC,
    TYC,
    CPGX,
    GAS,
    TE,
    CVC,
    BXLT,
    CCE,
    ARG,
    TWC,
    SNDK,
    ADT,
    GME,
    THC,
    CAM,
    POM,
    ESV,
    GMCR,
    CNX,
    PCL,
    PCP,
    BRCM,
    ACE,
    FOSL,
    ALTR,
    CMCSK,
    CSC,
    SIAL,
    GNW,
    HCBK,
    JOY,
    HSP,
    PLL,
    DTV,
    NE,
    FDO,
    KRFT,
    ATI,
    TEG,
    QEP,
    LO,
    WIN,
    DNR,
    NBR,
    AVP,
    CFN,
    PETM,
    SWY,
    COV,
    BTU,
    GHC,
    RDC,
    X,
    FRX,
    IGT,
    LSI,
    BEAM,
    SLM,
    CLF,
    WPX,
    LIFE,
    ANF,
    JDSU,
    MOLX,
    JCP,
    NYX,
    DELL,
    SAI,
    BMC,
    S,
    APOL,
    FHN,
    HNZ,
    DF,
    CVH,
    PCS,
    BIG,
    FII,
    TIE,
    RRD,
    ANR,
    LXK,
    DV,
    SHLD,
    GR,
    PGN,
    SLE,
    NVLS,
    MMI,
    EP,
    SVU,
    MHS,
    CPWR,
    TLAB,
    AKS,
    MWW,
    WFR,
    JNS,
    ITT,
    CEPH,
    NSM,
    MI,
    RSH,
    MEE,
    NOVL,
    GENZ,
    Q,
    MFE,
    AYE,
    KG,
    EK,
    ODP,
    NYT,
    PTV,
    SII,
    MIL,
    STR,
    XTO,
    BJS,
    RX,
    CIEN,
    DYN,
    KBH,
    CVG,
    MBI,
    SGP,
    CBE,
    CTX,
    TLE,
    ACAS,
    JNY,
    WB,
    LEH,
    FRE,
    FNM,
    CFC,
    BC,
    OMX,
    ABK,
    TRB,
    DJ,
    AV,
    SLR,
    NCR,
    FDC,
    KSE,
    TSG,
    SBL,
    ABS,
    GLK,
    QTRN,
    BS,
    GRA,
    CCK,
    RAD,
    TMC,
    SMS,
    LDW,
    HPH,
    GRN,
    SUN,
    USL,
    CDAY,
    RE,
    WLTW,
    DISCK,
    FB,
    KORS,
    DLPH,
    JOYG,
    PCLN,
    HRS,
    COG,
    JEC,
    TSO,
    LUK,
    KFT,
    WLP,
    FSR,
}

pub type Result<T> = std::result::Result<T, Error>;

/// The API of SnP, which defines all the functionality of SnP modules.
pub trait Api {
    fn get_snp(&self, ticker: &str) -> Result<SnP>;
    fn get_symbol(&self, snp: &SnP) -> Arc<str>;
    fn get_snp_map(&self) -> HashMap<SnP, Data>;
    fn get_snp_by_date(&self, date: &NaiveDate) -> Vec<SnP>;
}

/// All possible errors of SnP modules.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    ParseError(#[from] chrono::ParseError),
    #[error("{0}")]
    FormatterError(String),
    #[error("{0}")]
    NotFound(String),
}
