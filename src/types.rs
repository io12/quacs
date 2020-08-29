use serde::{Deserialize, Serialize};

/// Enum of attributes which can be restricted.
#[derive(Serialize, Deserialize)]
pub enum CourseAttribute {
    /// Course must be marked at Comm Intensive
    CommunicationIntensive,

    /// Course must have been taken in this semester
    RequiredSemester,
}

/// Holds each semester a course can be offered in
#[derive(Serialize, Deserialize)]
pub enum Semester {
    Fall,
    Spring,
    Summer,
}

/// Restriction on the metadata over a course for a Course restriction.
#[derive(Serialize, Deserialize)]
pub enum CourseRestriction {
    /// Checks against a certain attribute of a course.
    Attribute {
        operator: BooleanOperator,
        attribute: CourseAttribute,
    },
}

/// Restricts courses over various fields.  If a field is `None`, that means it is unrestricted.
#[derive(Serialize, Deserialize)]
pub struct Course {
    pub dept: String,
    pub crse: String,
    pub restriction: Vec<CourseRestriction>,
    // TODO: what does `NewDiscipline` mean?
}

/// Operator used for comparing different values.  This is probably only going to be Equal,
/// but it's separated into an enum to allow for more extensibility.
#[derive(Serialize, Deserialize)]
pub enum BooleanOperator {
    Equal,
}

/// Enum specialized on each condition's type
#[derive(Serialize, Deserialize)]
pub enum IfCondition {
    /// A nested If condition
    Nested {
        connector: ConditionOperator,
        left_condition: Box<IfCondition>,
        right_condition: Box<IfCondition>,
    },
    /// Checks against the user's major
    Major {
        operator: BooleanOperator,
        major: String,
    },
    /// Checks against the user's degree
    Degree {
        operator: BooleanOperator,
        degree: String,
    },
}

#[derive(Serialize, Deserialize)]
pub enum ConditionOperator {
    Or,
    And,
}

/// Enum specialized based on each type of rule
#[derive(Serialize, Deserialize)]
pub enum RuleData {
    /// Conditional rule (e.g. "If you're major X")
    If {
        condition: IfCondition,
        if_branch: Vec<Rule>,
        else_branch: Vec<Rule>,
    },
    /// A certain number of course requirements must be met (e.g. the courses for the CSCI
    /// concentrations)
    Course {
        num_courses_needed: Option<usize>,
        num_credits_needed: Option<usize>,
        courses: Vec<Course>,
        except: Vec<Course>,
        // qualifier: TODO (XML key: 'Qualifier')
    },
    /// Group of rules which must have a certain number be met.
    /// Note: There's a Degreeworks RuleType "Subset" which is a `Group` where
    /// every rule must be met.
    Group {
        num_needed: usize, // number needed
        rules: Vec<Rule>,
    },
}

/// Wrapper for an individual rule
#[derive(Serialize, Deserialize)]
pub struct Rule {
    //pub per_complete: usize, // TODO: is this needed?
    pub label: String,
    pub rule_data: RuleData,
    pub extra_text: Option<String>,
}

/// Holds a collection of rules (e.g. "Communication Intensive Courses")
#[derive(Serialize, Deserialize)]
pub struct Block {
    pub rules: Vec<Rule>,
    pub title: String,
}
