use std::vec::Vec;

pub enum Request {
    Brew(BrewRequest),
    Monitor(MonitorRequestVersion),
    State(StateRequest),
    Parameter(ParameterRequest),
    Raw(Vec<u8>),
}

#[derive(Debug, PartialEq)]
pub enum Response {
    State(MonitorState),
    Raw(Vec<u8>),
}

pub enum MonitorRequestVersion {
    V0,
    V1,
    V2,
}

pub enum StateRequest {
    TurnOn,
}

pub enum BrewRequest {
    Coffee,
}

pub enum ParameterRequest {
    ReadParameter(ParameterId, u8),
    WriteParameter(ParameterId),
}

pub enum ParameterId {
    WaterHardness,
}

pub enum Strength {}

pub enum Size {}

#[derive(Debug, PartialEq)]
pub enum MachineState {
    StandBy,
    TurningOn,
    ShuttingDown,
    Descaling,
    SteamPreparation,
    Recovery,
    Ready,
    /// Working is ready w/a function progress
    Working,
    Rinsing,
    MilkPreparation,
    HotWaterDelivery,
    MilkCleaning,
    Unknown(u8),
}

pub enum Accessory {
    None,
    Water,
    Milk,
    Chocolate,
    MilkClean,
    Unknown(u8),
}

pub enum BeverageTasteType {
    Delete,                  // 0
    Save,                    // 1
    Prepare,                 // 2
    PrepareAndSave,          // 3
    SaveInversion,           // 5
    PrepareInversion,        // 6
    PrepareAndSaveInversion, // 7
}

pub enum Ingredients {
    Temp = 0,                  // TEMP
    Coffee = 1,                // COFFEE
    Taste = 2,                 // TASTE
    Granulometry = 3,          // GRANULOMETRY
    Blend = 4,                 // BLEND
    InfusionSpeed = 5,         // INFUSION_SPEED
    Preinfusion = 6,           // PREINFUSIONE
    Crema = 7,                 // CREMA
    DueXPer = 8,               // DUExPER
    Milk = 9,                  // MILK
    MilkTemp = 10,             // MILK_TEMP
    MilkFroth = 11,            // MILK_FROTH
    Inversion = 12,            // INVERSION
    TheTemp = 13,              // THE_TEMP
    TheProfile = 14,           // THE_PROFILE
    HotWater = 15,             // HOT_WATER
    MixVelocity = 16,          // MIX_VELOCITY
    MixDuration = 17,          // MIX_DURATION
    DensityMultiBeverage = 18, // DENSITY_MULTI_BEVERAGE
    TempMultiBeverage = 19,    // TEMP_MULTI_BEVERAGE
    DecalcType = 20,           // DECALC_TYPE
    TempRisciaquo = 21,        // TEMP_RISCIACQUO
    WaterRisciaquo = 22,       // WATER_RISCIACQUO
    CleanType = 23,            // CLEAN_TYPE
    Programmable = 24,         // PROGRAMABLE
    Visible = 25,              // VISIBLE
    VisibleInProgramming = 26, // VISIBLE_IN_PROGRAMMING
    IndexLength = 27,          // INDEX_LENGTH
    Accessorio = 28,           // ACCESSORIO
}

#[derive(Debug, PartialEq)]
pub struct MonitorState {
    pub state: MachineState,
    pub progress: u8,
    pub percentage: u8,
    pub load0: u8,
    pub load1: u8,
    pub raw: Vec<u8>,
}

impl Request {
    pub fn encode(&self) -> Vec<u8> {
        match self {
            Request::Brew(r) => r.encode(),
            Request::Monitor(r) => r.encode(),
            Request::State(r) => r.encode(),
            Request::Parameter(r) => r.encode(),

            Request::Raw(r) => r.clone(),
        }
    }
}

impl BrewRequest {
    pub fn encode(&self) -> Vec<u8> {
        // dispense request, 0xf0, beverage type, trigger, parameters*, taste type
        // parameter: coffee quantity, coffee aroma, water quantity, milk quantity, froth
        match *self {
            BrewRequest::Coffee => {
                vec![
                    0x83, 0xf0, 0x02, 0x01, 0x01, 0x00, 0x67, 0x02, 0x02, 0x00, 0x00, 0x06,
                ]
            }
        }
    }
}

impl MonitorRequestVersion {
    pub fn encode(&self) -> Vec<u8> {
        match *self {
            MonitorRequestVersion::V0 => {
                vec![0x60, 0x0f]
            }
            MonitorRequestVersion::V1 => {
                vec![0x70, 0x0f]
            }
            MonitorRequestVersion::V2 => {
                vec![0x75, 0x0f]
            }
        }
    }
}

impl ParameterRequest {
    pub fn encode(&self) -> Vec<u8> {
        unimplemented!();
    }
}

impl StateRequest {
    pub fn encode(&self) -> Vec<u8> {
        match *self {
            StateRequest::TurnOn => {
                vec![0x84, 0x0f, 0x02, 0x01]
            }
        }
    }
}

impl Response {
    pub fn decode(data: &[u8]) -> Self {
        if data[0] == 0x75 && data.len() > 10 {
            Response::State(MonitorState::decode(&data[2..]))
        } else {
            Response::Raw(data.to_vec())
        }
    }
}

impl MonitorState {
    pub fn decode(data: &[u8]) -> Self {
        /* accessory, sw0, sw1, sw2, sw3, function, function progress, percentage, ?, load0, load1, sw, water */

        // Handle ready/working overlap
        let mut state = MachineState::decode(data[5]);
        if state == MachineState::Ready && data[6] != 0 {
            state = MachineState::Working;
        }

        MonitorState {
            state: MachineState::decode(data[5]),
            progress: data[6],
            percentage: data[7],
            load0: data[8],
            load1: data[9],
            raw: data.to_vec(),
        }

        // progress 5 = water 3 = hot wter

        /*

            <string name="COFFEE_DISPENSING_STATUS_0">Ready to use</string>
            <string name="COFFEE_DISPENSING_STATUS_1">Select beverage</string>
            <string name="COFFEE_DISPENSING_STATUS_11">Delivery</string>
            <string name="COFFEE_DISPENSING_STATUS_14">Brewing unit moving</string>
            <string name="COFFEE_DISPENSING_STATUS_16">End</string>
            <string name="COFFEE_DISPENSING_STATUS_3">Brewing unit moving</string>
            <string name="COFFEE_DISPENSING_STATUS_4">Grinding</string>
            <string name="COFFEE_DISPENSING_STATUS_6">Brewing unit moving</string>
            <string name="COFFEE_DISPENSING_STATUS_7">Water heating up</string>
            <string name="COFFEE_DISPENSING_STATUS_8">Pre-infusion</string>
        */
    }
}

impl MachineState {
    pub fn decode(data: u8) -> Self {
        match data {
            0 => MachineState::StandBy,
            1 => MachineState::TurningOn,
            2 => MachineState::ShuttingDown,
            4 => MachineState::Descaling,
            5 => MachineState::SteamPreparation,
            6 => MachineState::Recovery,
            7 => MachineState::Ready,
            8 => MachineState::Rinsing,
            10 => MachineState::MilkPreparation,
            11 => MachineState::HotWaterDelivery,
            12 => MachineState::MilkCleaning,
            n => MachineState::Unknown(n),
        }
    }
}
