{
  "header": {
    "name": "String",
    "exposing": {
      "Just": [
        {
          "Type": "String"
        },
        {
          "Definition": "isEmpty"
        },
        {
          "Definition": "length"
        },
        {
          "Definition": "reverse"
        },
        {
          "Definition": "repeat"
        },
        {
          "Definition": "replace"
        },
        {
          "Definition": "append"
        },
        {
          "Definition": "concat"
        },
        {
          "Definition": "split"
        },
        {
          "Definition": "join"
        },
        {
          "Definition": "words"
        },
        {
          "Definition": "lines"
        },
        {
          "Definition": "slice"
        },
        {
          "Definition": "left"
        },
        {
          "Definition": "right"
        },
        {
          "Definition": "dropLeft"
        },
        {
          "Definition": "dropRight"
        },
        {
          "Definition": "contains"
        },
        {
          "Definition": "startsWith"
        },
        {
          "Definition": "endsWith"
        },
        {
          "Definition": "indexes"
        },
        {
          "Definition": "indices"
        },
        {
          "Definition": "toInt"
        },
        {
          "Definition": "fromInt"
        },
        {
          "Definition": "toFloat"
        },
        {
          "Definition": "fromFloat"
        },
        {
          "Definition": "fromChar"
        },
        {
          "Definition": "cons"
        },
        {
          "Definition": "uncons"
        },
        {
          "Definition": "toList"
        },
        {
          "Definition": "fromList"
        },
        {
          "Definition": "toUpper"
        },
        {
          "Definition": "toLower"
        },
        {
          "Definition": "pad"
        },
        {
          "Definition": "padLeft"
        },
        {
          "Definition": "padRight"
        },
        {
          "Definition": "trim"
        },
        {
          "Definition": "trimLeft"
        },
        {
          "Definition": "trimRight"
        },
        {
          "Definition": "map"
        },
        {
          "Definition": "filter"
        },
        {
          "Definition": "foldl"
        },
        {
          "Definition": "foldr"
        },
        {
          "Definition": "any"
        },
        {
          "Definition": "all"
        }
      ]
    }
  },
  "imports": [
    {
      "path": [
        "Basics"
      ],
      "alias": null,
      "exposing": "All"
    },
    {
      "path": [
        "Bitwise"
      ],
      "alias": null,
      "exposing": null
    },
    {
      "path": [
        "Char"
      ],
      "alias": null,
      "exposing": {
        "Just": [
          {
            "Type": "Char"
          }
        ]
      }
    },
    {
      "path": [
        "Elm",
        "Kernel",
        "List"
      ],
      "alias": null,
      "exposing": null
    },
    {
      "path": [
        "Elm",
        "Kernel",
        "String"
      ],
      "alias": null,
      "exposing": null
    },
    {
      "path": [
        "List"
      ],
      "alias": null,
      "exposing": {
        "Just": [
          {
            "BinaryOperator": "::"
          }
        ]
      }
    },
    {
      "path": [
        "Maybe"
      ],
      "alias": null,
      "exposing": {
        "Just": [
          {
            "Type": "Maybe"
          }
        ]
      }
    },
    {
      "path": [
        "Result"
      ],
      "alias": null,
      "exposing": {
        "Just": [
          {
            "Type": "Result"
          }
        ]
      }
    }
  ],
  "statements": [
    {
      "Adt": [
        "String",
        [],
        [
          [
            [
              2472,
              2478
            ],
            "String",
            []
          ]
        ]
      ]
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "String",
                []
              ]
            },
            {
              "Tag": [
                "Bool",
                []
              ]
            }
          ]
        },
        "name": "isEmpty",
        "patterns": [
          {
            "Var": [
              [
                2666,
                2672
              ],
              "string"
            ]
          }
        ],
        "expr": {
          "OpChain": [
            [
              2677,
              2689
            ],
            [
              {
                "Ref": [
                  [
                    2677,
                    2683
                  ],
                  "string"
                ]
              },
              {
                "Literal": [
                  [
                    2687,
                    2689
                  ],
                  {
                    "String": ""
                  }
                ]
              }
            ],
            [
              "=="
            ]
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "String",
                []
              ]
            },
            {
              "Tag": [
                "Int",
                []
              ]
            }
          ]
        },
        "name": "length",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              2813,
              2837
            ],
            [
              "Elm",
              "Kernel",
              "String"
            ],
            "length"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "String",
                []
              ]
            },
            {
              "Tag": [
                "String",
                []
              ]
            }
          ]
        },
        "name": "reverse",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              2942,
              2967
            ],
            [
              "Elm",
              "Kernel",
              "String"
            ],
            "reverse"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "Int",
                []
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "String",
                    []
                  ]
                },
                {
                  "Tag": [
                    "String",
                    []
                  ]
                }
              ]
            }
          ]
        },
        "name": "repeat",
        "patterns": [
          {
            "Var": [
              [
                3075,
                3076
              ],
              "n"
            ]
          },
          {
            "Var": [
              [
                3077,
                3082
              ],
              "chunk"
            ]
          }
        ],
        "expr": {
          "Application": [
            [
              3087,
              3108
            ],
            {
              "Application": [
                [
                  3087,
                  3108
                ],
                {
                  "Application": [
                    [
                      3087,
                      3108
                    ],
                    {
                      "Ref": [
                        [
                          3087,
                          3097
                        ],
                        "repeatHelp"
                      ]
                    },
                    {
                      "Ref": [
                        [
                          3098,
                          3099
                        ],
                        "n"
                      ]
                    }
                  ]
                },
                {
                  "Ref": [
                    [
                      3100,
                      3105
                    ],
                    "chunk"
                  ]
                }
              ]
            },
            {
              "Literal": [
                [
                  3106,
                  3108
                ],
                {
                  "String": ""
                }
              ]
            }
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "Int",
                []
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "String",
                    []
                  ]
                },
                {
                  "Fun": [
                    {
                      "Tag": [
                        "String",
                        []
                      ]
                    },
                    {
                      "Tag": [
                        "String",
                        []
                      ]
                    }
                  ]
                }
              ]
            }
          ]
        },
        "name": "repeatHelp",
        "patterns": [
          {
            "Var": [
              [
                3169,
                3170
              ],
              "n"
            ]
          },
          {
            "Var": [
              [
                3171,
                3176
              ],
              "chunk"
            ]
          },
          {
            "Var": [
              [
                3177,
                3183
              ],
              "result"
            ]
          }
        ],
        "expr": {
          "If": [
            [
              3188,
              3345
            ],
            {
              "OpChain": [
                [
                  3191,
                  3197
                ],
                [
                  {
                    "Ref": [
                      [
                        3191,
                        3192
                      ],
                      "n"
                    ]
                  },
                  {
                    "Literal": [
                      [
                        3196,
                        3197
                      ],
                      {
                        "Int": 0
                      }
                    ]
                  }
                ],
                [
                  "<="
                ]
              ]
            },
            {
              "Ref": [
                [
                  3207,
                  3213
                ],
                "result"
              ]
            },
            {
              "OpChain": [
                [
                  3225,
                  3345
                ],
                [
                  {
                    "Application": [
                      [
                        3225,
                        3279
                      ],
                      {
                        "Application": [
                          [
                            3225,
                            3279
                          ],
                          {
                            "Ref": [
                              [
                                3225,
                                3235
                              ],
                              "repeatHelp"
                            ]
                          },
                          {
                            "Application": [
                              [
                                3237,
                                3261
                              ],
                              {
                                "Application": [
                                  [
                                    3237,
                                    3261
                                  ],
                                  {
                                    "QualifiedRef": [
                                      [
                                        3237,
                                        3258
                                      ],
                                      [
                                        "Bitwise"
                                      ],
                                      "shiftRightBy"
                                    ]
                                  },
                                  {
                                    "Literal": [
                                      [
                                        3258,
                                        3259
                                      ],
                                      {
                                        "Int": 1
                                      }
                                    ]
                                  }
                                ]
                              },
                              {
                                "Ref": [
                                  [
                                    3260,
                                    3261
                                  ],
                                  "n"
                                ]
                              }
                            ]
                          }
                        ]
                      },
                      {
                        "OpChain": [
                          [
                            3264,
                            3278
                          ],
                          [
                            {
                              "Ref": [
                                [
                                  3264,
                                  3269
                                ],
                                "chunk"
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  3273,
                                  3278
                                ],
                                "chunk"
                              ]
                            }
                          ],
                          [
                            "++"
                          ]
                        ]
                      }
                    ]
                  },
                  {
                    "If": [
                      [
                        3289,
                        3345
                      ],
                      {
                        "OpChain": [
                          [
                            3292,
                            3312
                          ],
                          [
                            {
                              "Application": [
                                [
                                  3292,
                                  3307
                                ],
                                {
                                  "Application": [
                                    [
                                      3292,
                                      3307
                                    ],
                                    {
                                      "QualifiedRef": [
                                        [
                                          3292,
                                          3304
                                        ],
                                        [
                                          "Bitwise"
                                        ],
                                        "and"
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          3304,
                                          3305
                                        ],
                                        "n"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Literal": [
                                    [
                                      3306,
                                      3307
                                    ],
                                    {
                                      "Int": 1
                                    }
                                  ]
                                }
                              ]
                            },
                            {
                              "Literal": [
                                [
                                  3311,
                                  3312
                                ],
                                {
                                  "Int": 0
                                }
                              ]
                            }
                          ],
                          [
                            "=="
                          ]
                        ]
                      },
                      {
                        "Ref": [
                          [
                            3318,
                            3324
                          ],
                          "result"
                        ]
                      },
                      {
                        "OpChain": [
                          [
                            3330,
                            3345
                          ],
                          [
                            {
                              "Ref": [
                                [
                                  3330,
                                  3336
                                ],
                                "result"
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  3340,
                                  3345
                                ],
                                "chunk"
                              ]
                            }
                          ],
                          [
                            "++"
                          ]
                        ]
                      }
                    ]
                  }
                ],
                [
                  "<|"
                ]
              ]
            }
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "String",
                []
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "String",
                    []
                  ]
                },
                {
                  "Fun": [
                    {
                      "Tag": [
                        "String",
                        []
                      ]
                    },
                    {
                      "Tag": [
                        "String",
                        []
                      ]
                    }
                  ]
                }
              ]
            }
          ]
        },
        "name": "replace",
        "patterns": [
          {
            "Var": [
              [
                3774,
                3780
              ],
              "before"
            ]
          },
          {
            "Var": [
              [
                3781,
                3786
              ],
              "after"
            ]
          },
          {
            "Var": [
              [
                3787,
                3793
              ],
              "string"
            ]
          }
        ],
        "expr": {
          "Application": [
            [
              3798,
              3830
            ],
            {
              "Application": [
                [
                  3798,
                  3830
                ],
                {
                  "Ref": [
                    [
                      3798,
                      3802
                    ],
                    "join"
                  ]
                },
                {
                  "Ref": [
                    [
                      3803,
                      3808
                    ],
                    "after"
                  ]
                }
              ]
            },
            {
              "Application": [
                [
                  3810,
                  3829
                ],
                {
                  "Application": [
                    [
                      3810,
                      3829
                    ],
                    {
                      "Ref": [
                        [
                          3810,
                          3815
                        ],
                        "split"
                      ]
                    },
                    {
                      "Ref": [
                        [
                          3816,
                          3822
                        ],
                        "before"
                      ]
                    }
                  ]
                },
                {
                  "Ref": [
                    [
                      3823,
                      3829
                    ],
                    "string"
                  ]
                }
              ]
            }
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "String",
                []
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "String",
                    []
                  ]
                },
                {
                  "Tag": [
                    "String",
                    []
                  ]
                }
              ]
            }
          ]
        },
        "name": "append",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              4040,
              4064
            ],
            [
              "Elm",
              "Kernel",
              "String"
            ],
            "append"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "List",
                [
                  {
                    "Tag": [
                      "String",
                      []
                    ]
                  }
                ]
              ]
            },
            {
              "Tag": [
                "String",
                []
              ]
            }
          ]
        },
        "name": "concat",
        "patterns": [
          {
            "Var": [
              [
                4200,
                4207
              ],
              "strings"
            ]
          }
        ],
        "expr": {
          "Application": [
            [
              4212,
              4227
            ],
            {
              "Application": [
                [
                  4212,
                  4227
                ],
                {
                  "Ref": [
                    [
                      4212,
                      4216
                    ],
                    "join"
                  ]
                },
                {
                  "Literal": [
                    [
                      4217,
                      4219
                    ],
                    {
                      "String": ""
                    }
                  ]
                }
              ]
            },
            {
              "Ref": [
                [
                  4220,
                  4227
                ],
                "strings"
              ]
            }
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "String",
                []
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "String",
                    []
                  ]
                },
                {
                  "Tag": [
                    "List",
                    [
                      {
                        "Tag": [
                          "String",
                          []
                        ]
                      }
                    ]
                  ]
                }
              ]
            }
          ]
        },
        "name": "split",
        "patterns": [
          {
            "Var": [
              [
                4451,
                4454
              ],
              "sep"
            ]
          },
          {
            "Var": [
              [
                4455,
                4461
              ],
              "string"
            ]
          }
        ],
        "expr": {
          "Application": [
            [
              4466,
              4528
            ],
            {
              "QualifiedRef": [
                [
                  4466,
                  4492
                ],
                [
                  "Elm",
                  "Kernel",
                  "List"
                ],
                "fromArray"
              ]
            },
            {
              "Application": [
                [
                  4493,
                  4527
                ],
                {
                  "Application": [
                    [
                      4493,
                      4527
                    ],
                    {
                      "QualifiedRef": [
                        [
                          4493,
                          4517
                        ],
                        [
                          "Elm",
                          "Kernel",
                          "String"
                        ],
                        "split"
                      ]
                    },
                    {
                      "Ref": [
                        [
                          4517,
                          4520
                        ],
                        "sep"
                      ]
                    }
                  ]
                },
                {
                  "Ref": [
                    [
                      4521,
                      4527
                    ],
                    "string"
                  ]
                }
              ]
            }
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "String",
                []
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "List",
                    [
                      {
                        "Tag": [
                          "String",
                          []
                        ]
                      }
                    ]
                  ]
                },
                {
                  "Tag": [
                    "String",
                    []
                  ]
                }
              ]
            }
          ]
        },
        "name": "join",
        "patterns": [
          {
            "Var": [
              [
                4804,
                4807
              ],
              "sep"
            ]
          },
          {
            "Var": [
              [
                4808,
                4814
              ],
              "chunks"
            ]
          }
        ],
        "expr": {
          "Application": [
            [
              4819,
              4878
            ],
            {
              "Application": [
                [
                  4819,
                  4878
                ],
                {
                  "QualifiedRef": [
                    [
                      4819,
                      4842
                    ],
                    [
                      "Elm",
                      "Kernel",
                      "String"
                    ],
                    "join"
                  ]
                },
                {
                  "Ref": [
                    [
                      4842,
                      4845
                    ],
                    "sep"
                  ]
                }
              ]
            },
            {
              "Application": [
                [
                  4847,
                  4877
                ],
                {
                  "QualifiedRef": [
                    [
                      4847,
                      4871
                    ],
                    [
                      "Elm",
                      "Kernel",
                      "List"
                    ],
                    "toArray"
                  ]
                },
                {
                  "Ref": [
                    [
                      4871,
                      4877
                    ],
                    "chunks"
                  ]
                }
              ]
            }
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "String",
                []
              ]
            },
            {
              "Tag": [
                "List",
                [
                  {
                    "Tag": [
                      "String",
                      []
                    ]
                  }
                ]
              ]
            }
          ]
        },
        "name": "words",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              5060,
              5083
            ],
            [
              "Elm",
              "Kernel",
              "String"
            ],
            "words"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "String",
                []
              ]
            },
            {
              "Tag": [
                "List",
                [
                  {
                    "Tag": [
                      "String",
                      []
                    ]
                  }
                ]
              ]
            }
          ]
        },
        "name": "lines",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              5245,
              5268
            ],
            [
              "Elm",
              "Kernel",
              "String"
            ],
            "lines"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "Int",
                []
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "Int",
                    []
                  ]
                },
                {
                  "Fun": [
                    {
                      "Tag": [
                        "String",
                        []
                      ]
                    },
                    {
                      "Tag": [
                        "String",
                        []
                      ]
                    }
                  ]
                }
              ]
            }
          ]
        },
        "name": "slice",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              5651,
              5674
            ],
            [
              "Elm",
              "Kernel",
              "String"
            ],
            "slice"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "Int",
                []
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "String",
                    []
                  ]
                },
                {
                  "Tag": [
                    "String",
                    []
                  ]
                }
              ]
            }
          ]
        },
        "name": "left",
        "patterns": [
          {
            "Var": [
              [
                5801,
                5802
              ],
              "n"
            ]
          },
          {
            "Var": [
              [
                5803,
                5809
              ],
              "string"
            ]
          }
        ],
        "expr": {
          "If": [
            [
              5814,
              5862
            ],
            {
              "OpChain": [
                [
                  5817,
                  5822
                ],
                [
                  {
                    "Ref": [
                      [
                        5817,
                        5818
                      ],
                      "n"
                    ]
                  },
                  {
                    "Literal": [
                      [
                        5821,
                        5822
                      ],
                      {
                        "Int": 1
                      }
                    ]
                  }
                ],
                [
                  "<"
                ]
              ]
            },
            {
              "Literal": [
                [
                  5832,
                  5834
                ],
                {
                  "String": ""
                }
              ]
            },
            {
              "Application": [
                [
                  5846,
                  5862
                ],
                {
                  "Application": [
                    [
                      5846,
                      5862
                    ],
                    {
                      "Application": [
                        [
                          5846,
                          5862
                        ],
                        {
                          "Ref": [
                            [
                              5846,
                              5851
                            ],
                            "slice"
                          ]
                        },
                        {
                          "Literal": [
                            [
                              5852,
                              5853
                            ],
                            {
                              "Int": 0
                            }
                          ]
                        }
                      ]
                    },
                    {
                      "Ref": [
                        [
                          5854,
                          5855
                        ],
                        "n"
                      ]
                    }
                  ]
                },
                {
                  "Ref": [
                    [
                      5856,
                      5862
                    ],
                    "string"
                  ]
                }
              ]
            }
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "Int",
                []
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "String",
                    []
                  ]
                },
                {
                  "Tag": [
                    "String",
                    []
                  ]
                }
              ]
            }
          ]
        },
        "name": "right",
        "patterns": [
          {
            "Var": [
              [
                5993,
                5994
              ],
              "n"
            ]
          },
          {
            "Var": [
              [
                5995,
                6001
              ],
              "string"
            ]
          }
        ],
        "expr": {
          "If": [
            [
              6006,
              6069
            ],
            {
              "OpChain": [
                [
                  6009,
                  6014
                ],
                [
                  {
                    "Ref": [
                      [
                        6009,
                        6010
                      ],
                      "n"
                    ]
                  },
                  {
                    "Literal": [
                      [
                        6013,
                        6014
                      ],
                      {
                        "Int": 1
                      }
                    ]
                  }
                ],
                [
                  "<"
                ]
              ]
            },
            {
              "Literal": [
                [
                  6024,
                  6026
                ],
                {
                  "String": ""
                }
              ]
            },
            {
              "Application": [
                [
                  6038,
                  6069
                ],
                {
                  "Application": [
                    [
                      6038,
                      6069
                    ],
                    {
                      "Application": [
                        [
                          6038,
                          6069
                        ],
                        {
                          "Ref": [
                            [
                              6038,
                              6043
                            ],
                            "slice"
                          ]
                        },
                        {
                          "Application": [
                            [
                              6044,
                              6047
                            ],
                            {
                              "Ref": [
                                [
                                  6044,
                                  6045
                                ],
                                "__internal__minus"
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  6045,
                                  6046
                                ],
                                "n"
                              ]
                            }
                          ]
                        }
                      ]
                    },
                    {
                      "Application": [
                        [
                          6048,
                          6061
                        ],
                        {
                          "Ref": [
                            [
                              6048,
                              6054
                            ],
                            "length"
                          ]
                        },
                        {
                          "Ref": [
                            [
                              6055,
                              6061
                            ],
                            "string"
                          ]
                        }
                      ]
                    }
                  ]
                },
                {
                  "Ref": [
                    [
                      6063,
                      6069
                    ],
                    "string"
                  ]
                }
              ]
            }
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "Int",
                []
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "String",
                    []
                  ]
                },
                {
                  "Tag": [
                    "String",
                    []
                  ]
                }
              ]
            }
          ]
        },
        "name": "dropLeft",
        "patterns": [
          {
            "Var": [
              [
                6228,
                6229
              ],
              "n"
            ]
          },
          {
            "Var": [
              [
                6230,
                6236
              ],
              "string"
            ]
          }
        ],
        "expr": {
          "If": [
            [
              6241,
              6307
            ],
            {
              "OpChain": [
                [
                  6244,
                  6249
                ],
                [
                  {
                    "Ref": [
                      [
                        6244,
                        6245
                      ],
                      "n"
                    ]
                  },
                  {
                    "Literal": [
                      [
                        6248,
                        6249
                      ],
                      {
                        "Int": 1
                      }
                    ]
                  }
                ],
                [
                  "<"
                ]
              ]
            },
            {
              "Ref": [
                [
                  6259,
                  6265
                ],
                "string"
              ]
            },
            {
              "Application": [
                [
                  6277,
                  6307
                ],
                {
                  "Application": [
                    [
                      6277,
                      6307
                    ],
                    {
                      "Application": [
                        [
                          6277,
                          6307
                        ],
                        {
                          "Ref": [
                            [
                              6277,
                              6282
                            ],
                            "slice"
                          ]
                        },
                        {
                          "Ref": [
                            [
                              6283,
                              6284
                            ],
                            "n"
                          ]
                        }
                      ]
                    },
                    {
                      "Application": [
                        [
                          6286,
                          6299
                        ],
                        {
                          "Ref": [
                            [
                              6286,
                              6292
                            ],
                            "length"
                          ]
                        },
                        {
                          "Ref": [
                            [
                              6293,
                              6299
                            ],
                            "string"
                          ]
                        }
                      ]
                    }
                  ]
                },
                {
                  "Ref": [
                    [
                      6301,
                      6307
                    ],
                    "string"
                  ]
                }
              ]
            }
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "Int",
                []
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "String",
                    []
                  ]
                },
                {
                  "Tag": [
                    "String",
                    []
                  ]
                }
              ]
            }
          ]
        },
        "name": "dropRight",
        "patterns": [
          {
            "Var": [
              [
                6482,
                6483
              ],
              "n"
            ]
          },
          {
            "Var": [
              [
                6484,
                6490
              ],
              "string"
            ]
          }
        ],
        "expr": {
          "If": [
            [
              6495,
              6548
            ],
            {
              "OpChain": [
                [
                  6498,
                  6503
                ],
                [
                  {
                    "Ref": [
                      [
                        6498,
                        6499
                      ],
                      "n"
                    ]
                  },
                  {
                    "Literal": [
                      [
                        6502,
                        6503
                      ],
                      {
                        "Int": 1
                      }
                    ]
                  }
                ],
                [
                  "<"
                ]
              ]
            },
            {
              "Ref": [
                [
                  6513,
                  6519
                ],
                "string"
              ]
            },
            {
              "Application": [
                [
                  6531,
                  6548
                ],
                {
                  "Application": [
                    [
                      6531,
                      6548
                    ],
                    {
                      "Application": [
                        [
                          6531,
                          6548
                        ],
                        {
                          "Ref": [
                            [
                              6531,
                              6536
                            ],
                            "slice"
                          ]
                        },
                        {
                          "Literal": [
                            [
                              6537,
                              6538
                            ],
                            {
                              "Int": 0
                            }
                          ]
                        }
                      ]
                    },
                    {
                      "Application": [
                        [
                          6539,
                          6542
                        ],
                        {
                          "Ref": [
                            [
                              6539,
                              6540
                            ],
                            "__internal__minus"
                          ]
                        },
                        {
                          "Ref": [
                            [
                              6540,
                              6541
                            ],
                            "n"
                          ]
                        }
                      ]
                    }
                  ]
                },
                {
                  "Ref": [
                    [
                      6542,
                      6548
                    ],
                    "string"
                  ]
                }
              ]
            }
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "String",
                []
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "String",
                    []
                  ]
                },
                {
                  "Tag": [
                    "Bool",
                    []
                  ]
                }
              ]
            }
          ]
        },
        "name": "contains",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              6792,
              6818
            ],
            [
              "Elm",
              "Kernel",
              "String"
            ],
            "contains"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "String",
                []
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "String",
                    []
                  ]
                },
                {
                  "Tag": [
                    "Bool",
                    []
                  ]
                }
              ]
            }
          ]
        },
        "name": "startsWith",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              7011,
              7039
            ],
            [
              "Elm",
              "Kernel",
              "String"
            ],
            "startsWith"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "String",
                []
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "String",
                    []
                  ]
                },
                {
                  "Tag": [
                    "Bool",
                    []
                  ]
                }
              ]
            }
          ]
        },
        "name": "endsWith",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              7222,
              7248
            ],
            [
              "Elm",
              "Kernel",
              "String"
            ],
            "endsWith"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "String",
                []
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "String",
                    []
                  ]
                },
                {
                  "Tag": [
                    "List",
                    [
                      {
                        "Tag": [
                          "Int",
                          []
                        ]
                      }
                    ]
                  ]
                }
              ]
            }
          ]
        },
        "name": "indexes",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              7493,
              7518
            ],
            [
              "Elm",
              "Kernel",
              "String"
            ],
            "indexes"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "String",
                []
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "String",
                    []
                  ]
                },
                {
                  "Tag": [
                    "List",
                    [
                      {
                        "Tag": [
                          "Int",
                          []
                        ]
                      }
                    ]
                  ]
                }
              ]
            }
          ]
        },
        "name": "indices",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              7600,
              7625
            ],
            [
              "Elm",
              "Kernel",
              "String"
            ],
            "indexes"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "String",
                []
              ]
            },
            {
              "Tag": [
                "String",
                []
              ]
            }
          ]
        },
        "name": "toUpper",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              7824,
              7849
            ],
            [
              "Elm",
              "Kernel",
              "String"
            ],
            "toUpper"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "String",
                []
              ]
            },
            {
              "Tag": [
                "String",
                []
              ]
            }
          ]
        },
        "name": "toLower",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              8011,
              8036
            ],
            [
              "Elm",
              "Kernel",
              "String"
            ],
            "toLower"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "Int",
                []
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "Char",
                    []
                  ]
                },
                {
                  "Fun": [
                    {
                      "Tag": [
                        "String",
                        []
                      ]
                    },
                    {
                      "Tag": [
                        "String",
                        []
                      ]
                    }
                  ]
                }
              ]
            }
          ]
        },
        "name": "pad",
        "patterns": [
          {
            "Var": [
              [
                8238,
                8239
              ],
              "n"
            ]
          },
          {
            "Var": [
              [
                8240,
                8244
              ],
              "char"
            ]
          },
          {
            "Var": [
              [
                8245,
                8251
              ],
              "string"
            ]
          }
        ],
        "expr": {
          "Let": [
            [
              8256,
              8411
            ],
            [
              {
                "Def": {
                  "header": null,
                  "name": "half",
                  "patterns": [],
                  "expr": {
                    "OpChain": [
                      [
                        8277,
                        8315
                      ],
                      [
                        {
                          "Application": [
                            [
                              8277,
                              8311
                            ],
                            {
                              "QualifiedRef": [
                                [
                                  8277,
                                  8292
                                ],
                                [
                                  "Basics"
                                ],
                                "toFloat"
                              ]
                            },
                            {
                              "OpChain": [
                                [
                                  8293,
                                  8310
                                ],
                                [
                                  {
                                    "Ref": [
                                      [
                                        8293,
                                        8294
                                      ],
                                      "n"
                                    ]
                                  },
                                  {
                                    "Application": [
                                      [
                                        8297,
                                        8310
                                      ],
                                      {
                                        "Ref": [
                                          [
                                            8297,
                                            8303
                                          ],
                                          "length"
                                        ]
                                      },
                                      {
                                        "Ref": [
                                          [
                                            8304,
                                            8310
                                          ],
                                          "string"
                                        ]
                                      }
                                    ]
                                  }
                                ],
                                [
                                  "-"
                                ]
                              ]
                            }
                          ]
                        },
                        {
                          "Literal": [
                            [
                              8314,
                              8315
                            ],
                            {
                              "Int": 2
                            }
                          ]
                        }
                      ],
                      [
                        "/"
                      ]
                    ]
                  }
                }
              }
            ],
            {
              "OpChain": [
                [
                  8325,
                  8411
                ],
                [
                  {
                    "Application": [
                      [
                        8325,
                        8362
                      ],
                      {
                        "Application": [
                          [
                            8325,
                            8362
                          ],
                          {
                            "Ref": [
                              [
                                8325,
                                8331
                              ],
                              "repeat"
                            ]
                          },
                          {
                            "Application": [
                              [
                                8333,
                                8345
                              ],
                              {
                                "Ref": [
                                  [
                                    8333,
                                    8340
                                  ],
                                  "ceiling"
                                ]
                              },
                              {
                                "Ref": [
                                  [
                                    8341,
                                    8345
                                  ],
                                  "half"
                                ]
                              }
                            ]
                          }
                        ]
                      },
                      {
                        "Application": [
                          [
                            8348,
                            8361
                          ],
                          {
                            "Ref": [
                              [
                                8348,
                                8356
                              ],
                              "fromChar"
                            ]
                          },
                          {
                            "Ref": [
                              [
                                8357,
                                8361
                              ],
                              "char"
                            ]
                          }
                        ]
                      }
                    ]
                  },
                  {
                    "Ref": [
                      [
                        8366,
                        8372
                      ],
                      "string"
                    ]
                  },
                  {
                    "Application": [
                      [
                        8376,
                        8411
                      ],
                      {
                        "Application": [
                          [
                            8376,
                            8411
                          ],
                          {
                            "Ref": [
                              [
                                8376,
                                8382
                              ],
                              "repeat"
                            ]
                          },
                          {
                            "Application": [
                              [
                                8384,
                                8394
                              ],
                              {
                                "Ref": [
                                  [
                                    8384,
                                    8389
                                  ],
                                  "floor"
                                ]
                              },
                              {
                                "Ref": [
                                  [
                                    8390,
                                    8394
                                  ],
                                  "half"
                                ]
                              }
                            ]
                          }
                        ]
                      },
                      {
                        "Application": [
                          [
                            8397,
                            8410
                          ],
                          {
                            "Ref": [
                              [
                                8397,
                                8405
                              ],
                              "fromChar"
                            ]
                          },
                          {
                            "Ref": [
                              [
                                8406,
                                8410
                              ],
                              "char"
                            ]
                          }
                        ]
                      }
                    ]
                  }
                ],
                [
                  "++",
                  "++"
                ]
              ]
            }
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "Int",
                []
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "Char",
                    []
                  ]
                },
                {
                  "Fun": [
                    {
                      "Tag": [
                        "String",
                        []
                      ]
                    },
                    {
                      "Tag": [
                        "String",
                        []
                      ]
                    }
                  ]
                }
              ]
            }
          ]
        },
        "name": "padLeft",
        "patterns": [
          {
            "Var": [
              [
                8631,
                8632
              ],
              "n"
            ]
          },
          {
            "Var": [
              [
                8633,
                8637
              ],
              "char"
            ]
          },
          {
            "Var": [
              [
                8638,
                8644
              ],
              "string"
            ]
          }
        ],
        "expr": {
          "OpChain": [
            [
              8649,
              8701
            ],
            [
              {
                "Application": [
                  [
                    8649,
                    8691
                  ],
                  {
                    "Application": [
                      [
                        8649,
                        8691
                      ],
                      {
                        "Ref": [
                          [
                            8649,
                            8655
                          ],
                          "repeat"
                        ]
                      },
                      {
                        "OpChain": [
                          [
                            8657,
                            8674
                          ],
                          [
                            {
                              "Ref": [
                                [
                                  8657,
                                  8658
                                ],
                                "n"
                              ]
                            },
                            {
                              "Application": [
                                [
                                  8661,
                                  8674
                                ],
                                {
                                  "Ref": [
                                    [
                                      8661,
                                      8667
                                    ],
                                    "length"
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      8668,
                                      8674
                                    ],
                                    "string"
                                  ]
                                }
                              ]
                            }
                          ],
                          [
                            "-"
                          ]
                        ]
                      }
                    ]
                  },
                  {
                    "Application": [
                      [
                        8677,
                        8690
                      ],
                      {
                        "Ref": [
                          [
                            8677,
                            8685
                          ],
                          "fromChar"
                        ]
                      },
                      {
                        "Ref": [
                          [
                            8686,
                            8690
                          ],
                          "char"
                        ]
                      }
                    ]
                  }
                ]
              },
              {
                "Ref": [
                  [
                    8695,
                    8701
                  ],
                  "string"
                ]
              }
            ],
            [
              "++"
            ]
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "Int",
                []
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "Char",
                    []
                  ]
                },
                {
                  "Fun": [
                    {
                      "Tag": [
                        "String",
                        []
                      ]
                    },
                    {
                      "Tag": [
                        "String",
                        []
                      ]
                    }
                  ]
                }
              ]
            }
          ]
        },
        "name": "padRight",
        "patterns": [
          {
            "Var": [
              [
                8927,
                8928
              ],
              "n"
            ]
          },
          {
            "Var": [
              [
                8929,
                8933
              ],
              "char"
            ]
          },
          {
            "Var": [
              [
                8934,
                8940
              ],
              "string"
            ]
          }
        ],
        "expr": {
          "OpChain": [
            [
              8945,
              8997
            ],
            [
              {
                "Ref": [
                  [
                    8945,
                    8951
                  ],
                  "string"
                ]
              },
              {
                "Application": [
                  [
                    8955,
                    8997
                  ],
                  {
                    "Application": [
                      [
                        8955,
                        8997
                      ],
                      {
                        "Ref": [
                          [
                            8955,
                            8961
                          ],
                          "repeat"
                        ]
                      },
                      {
                        "OpChain": [
                          [
                            8963,
                            8980
                          ],
                          [
                            {
                              "Ref": [
                                [
                                  8963,
                                  8964
                                ],
                                "n"
                              ]
                            },
                            {
                              "Application": [
                                [
                                  8967,
                                  8980
                                ],
                                {
                                  "Ref": [
                                    [
                                      8967,
                                      8973
                                    ],
                                    "length"
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      8974,
                                      8980
                                    ],
                                    "string"
                                  ]
                                }
                              ]
                            }
                          ],
                          [
                            "-"
                          ]
                        ]
                      }
                    ]
                  },
                  {
                    "Application": [
                      [
                        8983,
                        8996
                      ],
                      {
                        "Ref": [
                          [
                            8983,
                            8991
                          ],
                          "fromChar"
                        ]
                      },
                      {
                        "Ref": [
                          [
                            8992,
                            8996
                          ],
                          "char"
                        ]
                      }
                    ]
                  }
                ]
              }
            ],
            [
              "++"
            ]
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "String",
                []
              ]
            },
            {
              "Tag": [
                "String",
                []
              ]
            }
          ]
        },
        "name": "trim",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              9122,
              9144
            ],
            [
              "Elm",
              "Kernel",
              "String"
            ],
            "trim"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "String",
                []
              ]
            },
            {
              "Tag": [
                "String",
                []
              ]
            }
          ]
        },
        "name": "trimLeft",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              9283,
              9309
            ],
            [
              "Elm",
              "Kernel",
              "String"
            ],
            "trimLeft"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "String",
                []
              ]
            },
            {
              "Tag": [
                "String",
                []
              ]
            }
          ]
        },
        "name": "trimRight",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              9450,
              9477
            ],
            [
              "Elm",
              "Kernel",
              "String"
            ],
            "trimRight"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "String",
                []
              ]
            },
            {
              "Tag": [
                "Maybe",
                [
                  {
                    "Tag": [
                      "Int",
                      []
                    ]
                  }
                ]
              ]
            }
          ]
        },
        "name": "toInt",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              10014,
              10037
            ],
            [
              "Elm",
              "Kernel",
              "String"
            ],
            "toInt"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "Int",
                []
              ]
            },
            {
              "Tag": [
                "String",
                []
              ]
            }
          ]
        },
        "name": "fromInt",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              10285,
              10313
            ],
            [
              "Elm",
              "Kernel",
              "String"
            ],
            "fromNumber"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "String",
                []
              ]
            },
            {
              "Tag": [
                "Maybe",
                [
                  {
                    "Tag": [
                      "Float",
                      []
                    ]
                  }
                ]
              ]
            }
          ]
        },
        "name": "toFloat",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              10882,
              10907
            ],
            [
              "Elm",
              "Kernel",
              "String"
            ],
            "toFloat"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "Float",
                []
              ]
            },
            {
              "Tag": [
                "String",
                []
              ]
            }
          ]
        },
        "name": "fromFloat",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              11200,
              11228
            ],
            [
              "Elm",
              "Kernel",
              "String"
            ],
            "fromNumber"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "String",
                []
              ]
            },
            {
              "Tag": [
                "List",
                [
                  {
                    "Tag": [
                      "Char",
                      []
                    ]
                  }
                ]
              ]
            }
          ]
        },
        "name": "toList",
        "patterns": [
          {
            "Var": [
              [
                11426,
                11432
              ],
              "string"
            ]
          }
        ],
        "expr": {
          "Application": [
            [
              11437,
              11457
            ],
            {
              "Application": [
                [
                  11437,
                  11457
                ],
                {
                  "Application": [
                    [
                      11437,
                      11457
                    ],
                    {
                      "Ref": [
                        [
                          11437,
                          11442
                        ],
                        "foldr"
                      ]
                    },
                    {
                      "Ref": [
                        [
                          11443,
                          11448
                        ],
                        "::"
                      ]
                    }
                  ]
                },
                {
                  "List": [
                    [
                      11448,
                      11451
                    ],
                    []
                  ]
                }
              ]
            },
            {
              "Ref": [
                [
                  11451,
                  11457
                ],
                "string"
              ]
            }
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "List",
                [
                  {
                    "Tag": [
                      "Char",
                      []
                    ]
                  }
                ]
              ]
            },
            {
              "Tag": [
                "String",
                []
              ]
            }
          ]
        },
        "name": "fromList",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              11745,
              11771
            ],
            [
              "Elm",
              "Kernel",
              "String"
            ],
            "fromList"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "Char",
                []
              ]
            },
            {
              "Tag": [
                "String",
                []
              ]
            }
          ]
        },
        "name": "fromChar",
        "patterns": [
          {
            "Var": [
              [
                11904,
                11908
              ],
              "char"
            ]
          }
        ],
        "expr": {
          "Application": [
            [
              11913,
              11925
            ],
            {
              "Application": [
                [
                  11913,
                  11925
                ],
                {
                  "Ref": [
                    [
                      11913,
                      11917
                    ],
                    "cons"
                  ]
                },
                {
                  "Ref": [
                    [
                      11918,
                      11922
                    ],
                    "char"
                  ]
                }
              ]
            },
            {
              "Literal": [
                [
                  11923,
                  11925
                ],
                {
                  "String": ""
                }
              ]
            }
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "Char",
                []
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "String",
                    []
                  ]
                },
                {
                  "Tag": [
                    "String",
                    []
                  ]
                }
              ]
            }
          ]
        },
        "name": "cons",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              12088,
              12110
            ],
            [
              "Elm",
              "Kernel",
              "String"
            ],
            "cons"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "String",
                []
              ]
            },
            {
              "Tag": [
                "Maybe",
                [
                  {
                    "Tuple": [
                      {
                        "Tag": [
                          "Char",
                          []
                        ]
                      },
                      {
                        "Tag": [
                          "String",
                          []
                        ]
                      }
                    ]
                  }
                ]
              ]
            }
          ]
        },
        "name": "uncons",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              12357,
              12381
            ],
            [
              "Elm",
              "Kernel",
              "String"
            ],
            "uncons"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Fun": [
                {
                  "Tag": [
                    "Char",
                    []
                  ]
                },
                {
                  "Tag": [
                    "Char",
                    []
                  ]
                }
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "String",
                    []
                  ]
                },
                {
                  "Tag": [
                    "String",
                    []
                  ]
                }
              ]
            }
          ]
        },
        "name": "map",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              12571,
              12592
            ],
            [
              "Elm",
              "Kernel",
              "String"
            ],
            "map"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Fun": [
                {
                  "Tag": [
                    "Char",
                    []
                  ]
                },
                {
                  "Tag": [
                    "Bool",
                    []
                  ]
                }
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "String",
                    []
                  ]
                },
                {
                  "Tag": [
                    "String",
                    []
                  ]
                }
              ]
            }
          ]
        },
        "name": "filter",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              12738,
              12762
            ],
            [
              "Elm",
              "Kernel",
              "String"
            ],
            "filter"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Fun": [
                {
                  "Tag": [
                    "Char",
                    []
                  ]
                },
                {
                  "Fun": [
                    {
                      "Var": "b"
                    },
                    {
                      "Var": "b"
                    }
                  ]
                }
              ]
            },
            {
              "Fun": [
                {
                  "Var": "b"
                },
                {
                  "Fun": [
                    {
                      "Tag": [
                        "String",
                        []
                      ]
                    },
                    {
                      "Var": "b"
                    }
                  ]
                }
              ]
            }
          ]
        },
        "name": "foldl",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              12894,
              12917
            ],
            [
              "Elm",
              "Kernel",
              "String"
            ],
            "foldl"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Fun": [
                {
                  "Tag": [
                    "Char",
                    []
                  ]
                },
                {
                  "Fun": [
                    {
                      "Var": "b"
                    },
                    {
                      "Var": "b"
                    }
                  ]
                }
              ]
            },
            {
              "Fun": [
                {
                  "Var": "b"
                },
                {
                  "Fun": [
                    {
                      "Tag": [
                        "String",
                        []
                      ]
                    },
                    {
                      "Var": "b"
                    }
                  ]
                }
              ]
            }
          ]
        },
        "name": "foldr",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              13050,
              13073
            ],
            [
              "Elm",
              "Kernel",
              "String"
            ],
            "foldr"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Fun": [
                {
                  "Tag": [
                    "Char",
                    []
                  ]
                },
                {
                  "Tag": [
                    "Bool",
                    []
                  ]
                }
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "String",
                    []
                  ]
                },
                {
                  "Tag": [
                    "Bool",
                    []
                  ]
                }
              ]
            }
          ]
        },
        "name": "any",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              13278,
              13299
            ],
            [
              "Elm",
              "Kernel",
              "String"
            ],
            "any"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Fun": [
                {
                  "Tag": [
                    "Char",
                    []
                  ]
                },
                {
                  "Tag": [
                    "Bool",
                    []
                  ]
                }
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "String",
                    []
                  ]
                },
                {
                  "Tag": [
                    "Bool",
                    []
                  ]
                }
              ]
            }
          ]
        },
        "name": "all",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              13505,
              13526
            ],
            [
              "Elm",
              "Kernel",
              "String"
            ],
            "all"
          ]
        }
      }
    }
  ]
}