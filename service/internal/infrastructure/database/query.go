package database

// Query represents a single SQL Query to execute against the database
type Query struct {
	sql   string
	binds map[string]interface{}
}

// NewBindsQuery creates a new Query object ready to execute against the database
func NewBindsQuery(sql string, binds map[string]interface{}) Query {
	return Query{
		sql:   sql,
		binds: binds,
	}
}

// NewQuery creates a new Query object ready to execute against the database
func NewQuery(sql string) Query {
	return NewBindsQuery(sql, map[string]interface{}{})
}
