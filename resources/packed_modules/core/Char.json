{
  "header": {
    "name": "Char",
    "exposing": {
      "Just": [
        {
          "Type": "Char"
        },
        {
          "Definition": "isUpper"
        },
        {
          "Definition": "isLower"
        },
        {
          "Definition": "isAlpha"
        },
        {
          "Definition": "isAlphaNum"
        },
        {
          "Definition": "isDigit"
        },
        {
          "Definition": "isOctDigit"
        },
        {
          "Definition": "isHexDigit"
        },
        {
          "Definition": "toUpper"
        },
        {
          "Definition": "toLower"
        },
        {
          "Definition": "toLocaleUpper"
        },
        {
          "Definition": "toLocaleLower"
        },
        {
          "Definition": "toCode"
        },
        {
          "Definition": "fromCode"
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
      "exposing": {
        "Just": [
          {
            "Type": "Bool"
          },
          {
            "Type": "Int"
          },
          {
            "BinaryOperator": "&&"
          },
          {
            "BinaryOperator": "||"
          },
          {
            "BinaryOperator": ">="
          },
          {
            "BinaryOperator": "<="
          }
        ]
      }
    },
    {
      "path": [
        "Elm",
        "Kernel",
        "Char"
      ],
      "alias": null,
      "exposing": null
    }
  ],
  "statements": [
    {
      "Adt": [
        "Char",
        [],
        [
          [
            [
              1315,
              1319
            ],
            "Char",
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
        "name": "isUpper",
        "patterns": [
          {
            "Var": [
              [
                1656,
                1660
              ],
              "char"
            ]
          }
        ],
        "expr": {
          "Let": [
            [
              1665,
              1735
            ],
            [
              {
                "Def": {
                  "header": null,
                  "name": "code",
                  "patterns": [],
                  "expr": {
                    "Application": [
                      [
                        1686,
                        1697
                      ],
                      {
                        "Ref": [
                          [
                            1686,
                            1692
                          ],
                          "toCode"
                        ]
                      },
                      {
                        "Ref": [
                          [
                            1693,
                            1697
                          ],
                          "char"
                        ]
                      }
                    ]
                  }
                }
              }
            ],
            {
              "OpChain": [
                [
                  1707,
                  1735
                ],
                [
                  {
                    "Ref": [
                      [
                        1707,
                        1711
                      ],
                      "code"
                    ]
                  },
                  {
                    "Literal": [
                      [
                        1715,
                        1719
                      ],
                      {
                        "Int": 90
                      }
                    ]
                  },
                  {
                    "Literal": [
                      [
                        1723,
                        1727
                      ],
                      {
                        "Int": 65
                      }
                    ]
                  },
                  {
                    "Ref": [
                      [
                        1731,
                        1735
                      ],
                      "code"
                    ]
                  }
                ],
                [
                  "<=",
                  "&&",
                  "<="
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
        "name": "isLower",
        "patterns": [
          {
            "Var": [
              [
                1995,
                1999
              ],
              "char"
            ]
          }
        ],
        "expr": {
          "Let": [
            [
              2004,
              2074
            ],
            [
              {
                "Def": {
                  "header": null,
                  "name": "code",
                  "patterns": [],
                  "expr": {
                    "Application": [
                      [
                        2025,
                        2036
                      ],
                      {
                        "Ref": [
                          [
                            2025,
                            2031
                          ],
                          "toCode"
                        ]
                      },
                      {
                        "Ref": [
                          [
                            2032,
                            2036
                          ],
                          "char"
                        ]
                      }
                    ]
                  }
                }
              }
            ],
            {
              "OpChain": [
                [
                  2046,
                  2074
                ],
                [
                  {
                    "Literal": [
                      [
                        2046,
                        2050
                      ],
                      {
                        "Int": 97
                      }
                    ]
                  },
                  {
                    "Ref": [
                      [
                        2054,
                        2058
                      ],
                      "code"
                    ]
                  },
                  {
                    "Ref": [
                      [
                        2062,
                        2066
                      ],
                      "code"
                    ]
                  },
                  {
                    "Literal": [
                      [
                        2070,
                        2074
                      ],
                      {
                        "Int": 122
                      }
                    ]
                  }
                ],
                [
                  "<=",
                  "&&",
                  "<="
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
        "name": "isAlpha",
        "patterns": [
          {
            "Var": [
              [
                2340,
                2344
              ],
              "char"
            ]
          }
        ],
        "expr": {
          "OpChain": [
            [
              2349,
              2377
            ],
            [
              {
                "Application": [
                  [
                    2349,
                    2361
                  ],
                  {
                    "Ref": [
                      [
                        2349,
                        2356
                      ],
                      "isLower"
                    ]
                  },
                  {
                    "Ref": [
                      [
                        2357,
                        2361
                      ],
                      "char"
                    ]
                  }
                ]
              },
              {
                "Application": [
                  [
                    2365,
                    2377
                  ],
                  {
                    "Ref": [
                      [
                        2365,
                        2372
                      ],
                      "isUpper"
                    ]
                  },
                  {
                    "Ref": [
                      [
                        2373,
                        2377
                      ],
                      "char"
                    ]
                  }
                ]
              }
            ],
            [
              "||"
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
        "name": "isAlphaNum",
        "patterns": [
          {
            "Var": [
              [
                2696,
                2700
              ],
              "char"
            ]
          }
        ],
        "expr": {
          "OpChain": [
            [
              2705,
              2749
            ],
            [
              {
                "Application": [
                  [
                    2705,
                    2717
                  ],
                  {
                    "Ref": [
                      [
                        2705,
                        2712
                      ],
                      "isLower"
                    ]
                  },
                  {
                    "Ref": [
                      [
                        2713,
                        2717
                      ],
                      "char"
                    ]
                  }
                ]
              },
              {
                "Application": [
                  [
                    2721,
                    2733
                  ],
                  {
                    "Ref": [
                      [
                        2721,
                        2728
                      ],
                      "isUpper"
                    ]
                  },
                  {
                    "Ref": [
                      [
                        2729,
                        2733
                      ],
                      "char"
                    ]
                  }
                ]
              },
              {
                "Application": [
                  [
                    2737,
                    2749
                  ],
                  {
                    "Ref": [
                      [
                        2737,
                        2744
                      ],
                      "isDigit"
                    ]
                  },
                  {
                    "Ref": [
                      [
                        2745,
                        2749
                      ],
                      "char"
                    ]
                  }
                ]
              }
            ],
            [
              "||",
              "||"
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
        "name": "isDigit",
        "patterns": [
          {
            "Var": [
              [
                2974,
                2978
              ],
              "char"
            ]
          }
        ],
        "expr": {
          "Let": [
            [
              2983,
              3053
            ],
            [
              {
                "Def": {
                  "header": null,
                  "name": "code",
                  "patterns": [],
                  "expr": {
                    "Application": [
                      [
                        3004,
                        3015
                      ],
                      {
                        "Ref": [
                          [
                            3004,
                            3010
                          ],
                          "toCode"
                        ]
                      },
                      {
                        "Ref": [
                          [
                            3011,
                            3015
                          ],
                          "char"
                        ]
                      }
                    ]
                  }
                }
              }
            ],
            {
              "OpChain": [
                [
                  3025,
                  3053
                ],
                [
                  {
                    "Ref": [
                      [
                        3025,
                        3029
                      ],
                      "code"
                    ]
                  },
                  {
                    "Literal": [
                      [
                        3033,
                        3037
                      ],
                      {
                        "Int": 57
                      }
                    ]
                  },
                  {
                    "Literal": [
                      [
                        3041,
                        3045
                      ],
                      {
                        "Int": 48
                      }
                    ]
                  },
                  {
                    "Ref": [
                      [
                        3049,
                        3053
                      ],
                      "code"
                    ]
                  }
                ],
                [
                  "<=",
                  "&&",
                  "<="
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
        "name": "isOctDigit",
        "patterns": [
          {
            "Var": [
              [
                3306,
                3310
              ],
              "char"
            ]
          }
        ],
        "expr": {
          "Let": [
            [
              3315,
              3385
            ],
            [
              {
                "Def": {
                  "header": null,
                  "name": "code",
                  "patterns": [],
                  "expr": {
                    "Application": [
                      [
                        3336,
                        3347
                      ],
                      {
                        "Ref": [
                          [
                            3336,
                            3342
                          ],
                          "toCode"
                        ]
                      },
                      {
                        "Ref": [
                          [
                            3343,
                            3347
                          ],
                          "char"
                        ]
                      }
                    ]
                  }
                }
              }
            ],
            {
              "OpChain": [
                [
                  3357,
                  3385
                ],
                [
                  {
                    "Ref": [
                      [
                        3357,
                        3361
                      ],
                      "code"
                    ]
                  },
                  {
                    "Literal": [
                      [
                        3365,
                        3369
                      ],
                      {
                        "Int": 55
                      }
                    ]
                  },
                  {
                    "Literal": [
                      [
                        3373,
                        3377
                      ],
                      {
                        "Int": 48
                      }
                    ]
                  },
                  {
                    "Ref": [
                      [
                        3381,
                        3385
                      ],
                      "code"
                    ]
                  }
                ],
                [
                  "<=",
                  "&&",
                  "<="
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
        "name": "isHexDigit",
        "patterns": [
          {
            "Var": [
              [
                3483,
                3487
              ],
              "char"
            ]
          }
        ],
        "expr": {
          "Let": [
            [
              3492,
              3640
            ],
            [
              {
                "Def": {
                  "header": null,
                  "name": "code",
                  "patterns": [],
                  "expr": {
                    "Application": [
                      [
                        3513,
                        3524
                      ],
                      {
                        "Ref": [
                          [
                            3513,
                            3519
                          ],
                          "toCode"
                        ]
                      },
                      {
                        "Ref": [
                          [
                            3520,
                            3524
                          ],
                          "char"
                        ]
                      }
                    ]
                  }
                }
              }
            ],
            {
              "OpChain": [
                [
                  3535,
                  3639
                ],
                [
                  {
                    "OpChain": [
                      [
                        3535,
                        3563
                      ],
                      [
                        {
                          "Literal": [
                            [
                              3535,
                              3539
                            ],
                            {
                              "Int": 48
                            }
                          ]
                        },
                        {
                          "Ref": [
                            [
                              3543,
                              3547
                            ],
                            "code"
                          ]
                        },
                        {
                          "Ref": [
                            [
                              3551,
                              3555
                            ],
                            "code"
                          ]
                        },
                        {
                          "Literal": [
                            [
                              3559,
                              3563
                            ],
                            {
                              "Int": 57
                            }
                          ]
                        }
                      ],
                      [
                        "<=",
                        "&&",
                        "<="
                      ]
                    ]
                  },
                  {
                    "OpChain": [
                      [
                        3573,
                        3601
                      ],
                      [
                        {
                          "Literal": [
                            [
                              3573,
                              3577
                            ],
                            {
                              "Int": 65
                            }
                          ]
                        },
                        {
                          "Ref": [
                            [
                              3581,
                              3585
                            ],
                            "code"
                          ]
                        },
                        {
                          "Ref": [
                            [
                              3589,
                              3593
                            ],
                            "code"
                          ]
                        },
                        {
                          "Literal": [
                            [
                              3597,
                              3601
                            ],
                            {
                              "Int": 70
                            }
                          ]
                        }
                      ],
                      [
                        "<=",
                        "&&",
                        "<="
                      ]
                    ]
                  },
                  {
                    "OpChain": [
                      [
                        3611,
                        3639
                      ],
                      [
                        {
                          "Literal": [
                            [
                              3611,
                              3615
                            ],
                            {
                              "Int": 97
                            }
                          ]
                        },
                        {
                          "Ref": [
                            [
                              3619,
                              3623
                            ],
                            "code"
                          ]
                        },
                        {
                          "Ref": [
                            [
                              3627,
                              3631
                            ],
                            "code"
                          ]
                        },
                        {
                          "Literal": [
                            [
                              3635,
                              3639
                            ],
                            {
                              "Int": 102
                            }
                          ]
                        }
                      ],
                      [
                        "<=",
                        "&&",
                        "<="
                      ]
                    ]
                  }
                ],
                [
                  "||",
                  "||"
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
        "name": "toUpper",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              3726,
              3749
            ],
            [
              "Elm",
              "Kernel",
              "Char"
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
        "name": "toLower",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              3817,
              3840
            ],
            [
              "Elm",
              "Kernel",
              "Char"
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
        "name": "toLocaleUpper",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              3968,
              3997
            ],
            [
              "Elm",
              "Kernel",
              "Char"
            ],
            "toLocaleUpper"
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
                "Char",
                []
              ]
            }
          ]
        },
        "name": "toLocaleLower",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              4125,
              4154
            ],
            [
              "Elm",
              "Kernel",
              "Char"
            ],
            "toLocaleLower"
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
                "Int",
                []
              ]
            }
          ]
        },
        "name": "toCode",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              4427,
              4449
            ],
            [
              "Elm",
              "Kernel",
              "Char"
            ],
            "toCode"
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
                "Char",
                []
              ]
            }
          ]
        },
        "name": "fromCode",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              4988,
              5012
            ],
            [
              "Elm",
              "Kernel",
              "Char"
            ],
            "fromCode"
          ]
        }
      }
    }
  ]
}