use std::convert::TryFrom;

#[repr(usize)]
#[derive(Clone, Debug, PartialEq)]
pub enum Parameter {
    /// Time (in units of MJD TDB).
    Time = 0,
    /// Simulation time (in units of scaled time).
    Tau = 1,
    /// X-coordinate in km.
    X = 2,
    /// Y-coordinate in km.
    Y = 3,
    /// Z-coordinate in km.
    Z = 4,
    /// X-velocity in km/s.
    VX = 5,
    /// Y-velocity in km/s.
    VY = 6,
    /// Z-velocity in km/s.
    VZ = 7,
    /// X-acceleration in km/s².
    AX = 8,
    /// Y-acceleration in km/s².
    AY = 9,
    /// Z-acceleration in km/s².
    AZ = 10,
}

impl Parameter {
    pub const MAX_PARAMETERS: usize = 32;

    /// Parameter index.
    pub const fn index(self) -> usize {
        self as usize
    }

    /// Parameter name.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Time => "Time",
            Self::Tau => "Tau",
            Self::X => "X0",
            Self::Y => "Y0",
            Self::Z => "Z0",
            Self::VX => "VX0",
            Self::VY => "VY0",
            Self::VZ => "VZ0",
            Self::AX => "AX0",
            Self::AY => "AY0",
            Self::AZ => "AZ0",
        }
    }
}

impl TryFrom<usize> for Parameter {
    type Error = ();

    fn try_from(i: usize) -> Result<Self, Self::Error> {
        match i {
            0 => Ok(Parameter::Time),
            1 => Ok(Parameter::Tau),
            2 => Ok(Parameter::X),
            3 => Ok(Parameter::Y),
            4 => Ok(Parameter::Z),
            5 => Ok(Parameter::VX),
            6 => Ok(Parameter::VY),
            7 => Ok(Parameter::VZ),
            8 => Ok(Parameter::AX),
            9 => Ok(Parameter::AY),
            10 => Ok(Parameter::AZ),
            _ => Err(()),
        }
    }
}

pub struct ParameterConfig<const ORDER: usize> {
    /// Bitmask indicating which parameters are being tracked.
    active_mask: u128,
    /// Number of parameters currently being tracked.
    parameter_count: usize,
    /// Index mapping of parameter index to active index within derivative arrays.
    index_map: Vec<Option<usize>>,
    /// Reverse mapping of parameter to active index.
    reverse_map: Vec<Parameter>,
}

impl<const ORDER: usize> ParameterConfig<ORDER> {
    pub fn new() -> Self {
        Self {
            active_mask: 0,
            parameter_count: 0,
            index_map: vec![None; Parameter::MAX_PARAMETERS],
            reverse_map: Vec::<Parameter>::new(),
        }
    }

    /// The number of tracked parameters.
    pub const fn parameter_count(&self) -> usize {
        self.parameter_count
    }

    /// Add a parameter to the set of tracked parameters.
    fn add_parameter(mut self, p: Parameter) -> Self {
        let bit = 1u128 << p.clone().index();
        if (self.active_mask & bit) == 0 {
            self.active_mask |= bit;
            self.index_map[p.clone().index()] = Some(self.parameter_count);
            self.reverse_map.push(p.clone());
            self.parameter_count += 1;
        }
        self
    }

    /// Get the active index for a given parameter.
    pub fn get_index(&self, p: Parameter) -> Option<usize> {
        self.index_map[p.clone().index()]
    }

    /// Get the parameter for a given index.
    pub fn get_parameter(&self, i: usize) -> Option<Parameter> {
        if i > self.parameter_count() {
            None
        } else {
            Some(self.reverse_map[i].clone())
        }
    }

    /// Return the currently tracked parameters.
    pub fn active_parameters(&self) -> Vec<Parameter> {
        let mut parameters = Vec::with_capacity(self.parameter_count);
        for i in 0..Parameter::MAX_PARAMETERS {
            if (self.active_mask & (1u128 << i)) != 0 {
                parameters.push(Parameter::try_from(i).unwrap());
            }
        }
        parameters
    }

    /// Track the given parameter.
    pub fn with_parameter(self, p: Parameter) -> Self {
        self.add_parameter(p)
    }

    /// Track derivatives with respect to time.
    pub fn with_time(self) -> Self {
        self.add_parameter(Parameter::Time)
    }

    /// Track derivatives with respect to the initial state.
    pub fn with_initial_state(self) -> Self {
        self.add_parameter(Parameter::X)
            .add_parameter(Parameter::Y)
            .add_parameter(Parameter::Z)
            .add_parameter(Parameter::VX)
            .add_parameter(Parameter::VY)
            .add_parameter(Parameter::VZ)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from() {
        for p in [
            Parameter::Time,
            Parameter::Tau,
            Parameter::X,
            Parameter::Y,
            Parameter::Z,
            Parameter::VX,
            Parameter::VY,
            Parameter::VZ,
            Parameter::AX,
            Parameter::AY,
            Parameter::AZ,
        ] {
            assert_eq!(p.clone(), Parameter::try_from(p.index()).unwrap())
        }
    }

    #[test]
    pub fn test_add_parameter() {
        let config = ParameterConfig::<2>::new()
            .add_parameter(Parameter::Time)
            .add_parameter(Parameter::AX);

        assert_eq!(config.active_mask, 1 + 256);
        assert_eq!(config.parameter_count(), 2);
        assert_eq!(config.index_map[0], Some(0));
        assert_eq!(config.index_map[8], Some(1));
        assert_eq!(config.reverse_map[0], Parameter::Time);
        assert_eq!(config.reverse_map[1], Parameter::AX);
    }

    #[test]
    pub fn test_with_parameter() {
        let config = ParameterConfig::<2>::new()
            .with_parameter(Parameter::Time)
            .with_parameter(Parameter::AX)
            .with_parameter(Parameter::X);

        assert_eq!(config.active_mask, 1 + 256 + 4);
        assert_eq!(config.parameter_count(), 3);
        assert_eq!(config.index_map[0], Some(0));
        assert_eq!(config.index_map[8], Some(1));
        assert_eq!(config.index_map[2], Some(2));
        assert_eq!(config.reverse_map[0], Parameter::Time);
        assert_eq!(config.reverse_map[1], Parameter::AX);
        assert_eq!(config.reverse_map[2], Parameter::X);
    }

    #[test]
    pub fn test_with_time() {
        let config = ParameterConfig::<2>::new().with_time();
        assert_eq!(config.active_mask, 1);
        assert_eq!(config.parameter_count(), 1);
        assert_eq!(config.index_map[0], Some(0));
        assert_eq!(config.reverse_map[0], Parameter::Time);
    }

    #[test]
    pub fn test_with_initial_state() {
        let config = ParameterConfig::<2>::new().with_initial_state();
        assert_eq!(config.active_mask, 4 + 8 + 16 + 32 + 64 + 128);
        assert_eq!(config.parameter_count(), 6);
        assert_eq!(config.index_map[2], Some(0));
        assert_eq!(config.index_map[3], Some(1));
        assert_eq!(config.index_map[4], Some(2));
        assert_eq!(config.index_map[5], Some(3));
        assert_eq!(config.index_map[6], Some(4));
        assert_eq!(config.index_map[7], Some(5));
        assert_eq!(config.reverse_map[0], Parameter::X);
        assert_eq!(config.reverse_map[1], Parameter::Y);
        assert_eq!(config.reverse_map[2], Parameter::Z);
        assert_eq!(config.reverse_map[3], Parameter::VX);
        assert_eq!(config.reverse_map[4], Parameter::VY);
        assert_eq!(config.reverse_map[5], Parameter::VZ);
    }
}
